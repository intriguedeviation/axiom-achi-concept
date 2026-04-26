import React from 'react';

const radians = (v) => v * Math.PI / 180.0;

const createPolygon = (data) => {
  const {cx, cy, radius, sides, rotation} = data;
  const circumference = 360.0;
  const sliceAngle = circumference / sides;
  const rads = radians(sliceAngle);
  const angleRads = radians(rotation);

  const pairs = [...Array(sides + 1).keys()].map((i) => ({
    x: cx + radius * Math.cos(i * rads + angleRads),
    y: cy + radius * Math.sin(i * rads + angleRads),
  }));
  pairs.shift();

  return pairs;
};

export const Board = ({scale}) => {
  const radius = scale;
  const cx = scale;
  const cy = scale;
  const outerCircleRadius = radius * 0.95;
  const innerCircleRadius = radius * 0.9;

  const gridRadius = radius * 0.85;

  const polygonParameters = {
    cx,
    cy,
    radius: gridRadius,
    sides: 4,
    rotation: 0
  };

  const outerPerimeterCoordPairs = createPolygon(polygonParameters);
  const firstPair = outerPerimeterCoordPairs[0]
  
  const placementCoordPairs = outerPerimeterCoordPairs.map((v, i, arr) => [
    v,
    arr[(i + 1) % arr.length],
  ]).map(([{ x: x1, y: y1 }, { x: x2, y: y2 }]) => ({
    x: (x1 + x2) * 0.5,
    y: (y1 + y2) * 0.5,
  }));

  let lineSet1 = outerPerimeterCoordPairs
    .slice(0, outerPerimeterCoordPairs.length / 2)
    .map((v, i) => [v, outerPerimeterCoordPairs[i + 2]]);

  let lineSet2 = placementCoordPairs
    .slice(0, placementCoordPairs.length / 2)
    .map((v , i) => [v, placementCoordPairs[i + 2]]);

  let circleSet = [...outerPerimeterCoordPairs, ...placementCoordPairs, ...[{x: cx, y: cy}]]
    .map(({x, y}) => ({cx: x, cy: y, r: 20, fill: '#fff', stroke: 'none'}))
    .map((props, i) => <circle {...props} key={i} />)
    ;

  let lineSet = [...lineSet1, ...lineSet2]
    .map(([{ x: x1, y: y1 }, { x: x2, y: y2 }]) => ({x1, y1, x2, y2}))
    .map((ls, i) => <line {...ls} stroke="#fff" key={i} />);
  
  const pathData = `M ${firstPair.x} ${firstPair.y} ` + outerPerimeterCoordPairs.map(p => `L ${p.x} ${p.y}`).join(' ') + ' Z';
  
  const sharedCircleProps = {
    stroke: 'none',
    fill: 'rgba(0, 0, 0, 0.5)',
  };

  const outerCircleProps = {
    cx: cx,
    cy: cy,
    r: outerCircleRadius,
    ...sharedCircleProps,
  };

  const innerCircleProps = {
    cx,
    cy,
    r: innerCircleRadius,
    ...sharedCircleProps,
  };

  return (
    <div className="game">
      <svg xmlns="http://www.w3.org/2000/svg" width={radius * 2} height={radius * 2} version="1.1">
        <defs>
          <filter id="filter-L3MLRi5vUXhBPmNcDK0KCw">
            <feGaussianBlur in="SourceGraphic" stdDeviation="3.2" />
            <feMerge>
              <feMergeNode />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>
        <g filter="url(#filter-L3MLRi5vUXhBPmNcDK0KCw)">
          <circle {...outerCircleProps} />
          <circle {...innerCircleProps} />
          <path d={pathData} stroke="#fff" fill="none" />
          {lineSet}
        </g>
        {circleSet}
      </svg>
    </div>
  );
};