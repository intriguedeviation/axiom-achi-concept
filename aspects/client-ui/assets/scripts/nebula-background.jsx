import React from 'react';

const vertexShaderSource = `
attribute vec2 aPosition;

void main() {
  gl_Position = vec4(aPosition, 0.0, 1.0);
}
`;

const fragmentShaderSource = `
precision highp float;

uniform vec2 uResolution;
uniform float uTime;
uniform float uMotion;

float hash(vec2 p) {
  p = fract(p * vec2(123.34, 456.21));
  p += dot(p, p + 45.32);
  return fract(p.x * p.y);
}

float noise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  vec2 u = f * f * (3.0 - 2.0 * f);

  return mix(
    mix(hash(i + vec2(0.0, 0.0)), hash(i + vec2(1.0, 0.0)), u.x),
    mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x),
    u.y
  );
}

float fbm(vec2 p) {
  float value = 0.0;
  float amplitude = 0.55;

  for (int i = 0; i < 6; i++) {
    value += amplitude * noise(p);
    p = mat2(1.62, -1.18, 1.18, 1.62) * p + 13.7;
    amplitude *= 0.48;
  }

  return value;
}

void main() {
  vec2 uv = (gl_FragCoord.xy - 0.5 * uResolution.xy) / min(uResolution.x, uResolution.y);
  float time = uTime * uMotion;

  vec2 drift = vec2(time * 0.018, -time * 0.012);
  float filament = fbm(uv * 2.15 + drift);
  float dust = fbm(uv * 5.4 - drift * 1.7);
  float fine = fbm(uv * 12.0 + vec2(time * 0.02));

  float radial = smoothstep(1.15, 0.08, length(uv + vec2(0.08, -0.04)));
  float cloud = smoothstep(0.32, 1.0, filament * 0.78 + dust * 0.36 + radial * 0.42);
  float lanes = smoothstep(0.28, 0.72, dust) * (1.0 - smoothstep(0.48, 0.92, fine));

  vec3 hAlpha = vec3(1.0, 0.08, 0.03);
  vec3 sulfurII = vec3(0.95, 0.21, 0.04);
  vec3 oxygenIII = vec3(0.08, 0.82, 0.95);
  vec3 hBeta = vec3(0.20, 0.36, 1.0);

  vec3 emission = hAlpha * cloud;
  emission += sulfurII * smoothstep(0.52, 0.95, filament + radial * 0.18) * 0.55;
  emission += oxygenIII * smoothstep(0.58, 1.0, dust + radial * 0.2) * 0.42;
  emission += hBeta * smoothstep(0.72, 1.0, fine + filament * 0.25) * 0.18;

  vec3 darkDust = vec3(0.018, 0.016, 0.022) * lanes * 1.65;
  vec3 deepSpace = vec3(0.003, 0.005, 0.012);
  vec3 color = deepSpace + emission * (0.32 + radial * 0.7) - darkDust;

  float stars = smoothstep(0.996, 1.0, hash(floor(gl_FragCoord.xy * 0.72)));
  stars *= 0.28 + 0.72 * hash(floor(gl_FragCoord.yx * 0.31));
  color += vec3(stars) * 0.75;

  color = max(color, vec3(0.0));
  color = vec3(1.0) - exp(-color * 1.55);
  color = pow(color, vec3(1.0 / 2.2));

  gl_FragColor = vec4(color, 1.0);
}
`;

const createShader = (gl, type, source) => {
  const shader = gl.createShader(type);
  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const message = gl.getShaderInfoLog(shader);
    gl.deleteShader(shader);
    throw new Error(message);
  }

  return shader;
};

const createProgram = (gl) => {
  const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
  const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
  const program = gl.createProgram();

  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);
  gl.deleteShader(vertexShader);
  gl.deleteShader(fragmentShader);

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    const message = gl.getProgramInfoLog(program);
    gl.deleteProgram(program);
    throw new Error(message);
  }

  return program;
};

export const NebulaBackground = () => {
  const canvasRef = React.useRef(null);

  React.useEffect(() => {
    const canvas = canvasRef.current;
    const gl = canvas?.getContext('webgl', {
      alpha: false,
      antialias: false,
      depth: false,
      stencil: false,
      powerPreference: 'high-performance',
    });

    if (!canvas || !gl) {
      canvas?.classList.add('nebula-background--fallback');
      return undefined;
    }

    const positionBuffer = gl.createBuffer();
    let program = null;

    try {
      program = createProgram(gl);
    } catch {
      canvas.classList.add('nebula-background--fallback');
      return undefined;
    }

    if (!positionBuffer) {
      canvas.classList.add('nebula-background--fallback');
      gl.deleteProgram(program);
      return undefined;
    }

    const positionLocation = gl.getAttribLocation(program, 'aPosition');
    const resolutionLocation = gl.getUniformLocation(program, 'uResolution');
    const timeLocation = gl.getUniformLocation(program, 'uTime');
    const motionLocation = gl.getUniformLocation(program, 'uMotion');
    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
    let frameId = 0;
    let width = 0;
    let height = 0;

    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
      gl.STATIC_DRAW,
    );
    gl.useProgram(program);
    gl.enableVertexAttribArray(positionLocation);
    gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

    const resize = () => {
      const pixelRatio = Math.min(window.devicePixelRatio || 1, 2);
      const nextWidth = Math.max(1, Math.floor(canvas.clientWidth * pixelRatio));
      const nextHeight = Math.max(1, Math.floor(canvas.clientHeight * pixelRatio));

      if (nextWidth !== width || nextHeight !== height) {
        width = nextWidth;
        height = nextHeight;
        canvas.width = width;
        canvas.height = height;
        gl.viewport(0, 0, width, height);
      }
    };

    const render = (time) => {
      resize();
      gl.useProgram(program);
      gl.uniform2f(resolutionLocation, width, height);
      gl.uniform1f(timeLocation, time * 0.001);
      gl.uniform1f(motionLocation, reducedMotion.matches ? 0.0 : 1.0);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
      frameId = window.requestAnimationFrame(render);
    };

    window.addEventListener('resize', resize);
    frameId = window.requestAnimationFrame(render);

    return () => {
      window.cancelAnimationFrame(frameId);
      window.removeEventListener('resize', resize);
      gl.deleteBuffer(positionBuffer);
      gl.deleteProgram(program);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="nebula-background"
      aria-hidden="true"
    />
  );
};
