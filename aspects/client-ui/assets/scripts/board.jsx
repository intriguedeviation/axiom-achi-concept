import React from 'react';
import {BoardPhase, PlayerSide} from 'axiom-achi';

const BOARD_SIDE_COUNT = 4;
const BOARD_POSITION_COUNT = 9;
const FULL_TURN_DEGREES = 360;
const DEFAULT_SCALE = 395;

const toRadians = (degrees) => degrees * Math.PI / 180;

// Produces evenly spaced points on a rotated perimeter. The Achi grid uses
// four perimeter anchors, four edge midpoints, and one center point.
const createPerimeterPoints = ({center, radius, sides, rotation}) => {
  const sliceAngle = FULL_TURN_DEGREES / sides;
  const rotationRadians = toRadians(rotation);

  return Array.from({length: sides}, (_, index) => {
    const angle = toRadians(sliceAngle * index) + rotationRadians;

    return {
      x: center.x + radius * Math.cos(angle),
      y: center.y + radius * Math.sin(angle),
    };
  });
};

const midpoint = (a, b) => ({
  x: (a.x + b.x) * 0.5,
  y: (a.y + b.y) * 0.5,
});

const oppositeSegments = (points) => (
  points
    .slice(0, points.length / 2)
    .map((point, index) => [point, points[index + points.length / 2]])
);

const segmentProps = ([start, end]) => ({
  x1: start.x,
  y1: start.y,
  x2: end.x,
  y2: end.y,
});

const sideClass = (side) => {
  if (side === PlayerSide.Red) {
    return 'board__position--red';
  }
  if (side === PlayerSide.Black) {
    return 'board__position--black';
  }

  return '';
};

const sideLabel = (side) => side === PlayerSide.Red ? 'red' : 'black';

const positionLabel = ({index, occupant, selected, phase}) => {
  const positionNumber = index + 1;

  if (occupant) {
    const action = phase === BoardPhase.Movement ? 'select or move from' : 'occupied';
    return `${action} ${sideLabel(occupant.side)} token ${occupant.tokenIndex + 1} at position ${positionNumber}`;
  }

  if (selected) {
    return `move selected token to empty position ${positionNumber}`;
  }

  return `place token at empty position ${positionNumber}`;
};

const polygonPath = (points) => {
  const [firstPoint, ...remainingPoints] = points;
  const segments = remainingPoints.map((point) => `L ${point.x} ${point.y}`);

  return `M ${firstPoint.x} ${firstPoint.y} ${segments.join(' ')} Z`;
};

const createBoardGeometry = (scale) => {
  const radius = scale;
  const center = {x: scale, y: scale};
  const gridRadius = radius * 0.85;
  const outerPoints = createPerimeterPoints({
    center,
    radius: gridRadius,
    sides: BOARD_SIDE_COUNT,
    rotation: 0,
  });
  const edgePoints = outerPoints.map((point, index, points) => (
    midpoint(point, points[(index + 1) % points.length])
  ));
  const positionPoints = [...outerPoints, ...edgePoints, center];
  const boardSegments = [
    ...oppositeSegments(outerPoints),
    ...oppositeSegments(edgePoints),
  ];

  return {
    center,
    viewBoxSize: radius * 2,
    outerCircleRadius: radius * 0.95,
    innerCircleRadius: radius * 0.9,
    perimeterPath: polygonPath(outerPoints),
    positionPoints,
    boardSegments,
  };
};

// TODO: fix physical location mapping to align with game grid
export const Board = ({
  activeSide,
  onPositionAction,
  phase,
  scale = DEFAULT_SCALE,
  selectedToken = null,
  tokens = [],
  victory = {achieved: false},
}) => {
  const reactId = React.useId();
  const glowFilterId = React.useMemo(
    () => `board-glow-${reactId.replaceAll(':', '')}`,
    [reactId],
  );
  const geometry = React.useMemo(() => createBoardGeometry(scale), [scale]);
  const tokenAtPosition = React.useCallback(
    (position) => tokens.find((token) => token.position === position),
    [tokens],
  );
  const handlePositionKeyDown = React.useCallback((event, position) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onPositionAction(position);
    }
  }, [onPositionAction]);

  return (
    <div className="game">
      <svg
        className="board"
        xmlns="http://www.w3.org/2000/svg"
        viewBox={`0 0 ${geometry.viewBoxSize} ${geometry.viewBoxSize}`}
        role="img"
        aria-label="Achi board with nine playable positions"
      >
        <title>Achi board</title>
        <defs>
          <filter id={glowFilterId}>
            <feGaussianBlur in="SourceGraphic" stdDeviation="3.2" />
            <feMerge>
              <feMergeNode />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>

        <g className="board__grid" filter={`url(#${glowFilterId})`}>
          <circle
            className="board__field"
            cx={geometry.center.x}
            cy={geometry.center.y}
            r={geometry.outerCircleRadius}
          />
          <circle
            className="board__field board__field--inner"
            cx={geometry.center.x}
            cy={geometry.center.y}
            r={geometry.innerCircleRadius}
          />
          <path className="board__perimeter" d={geometry.perimeterPath} />
          {geometry.boardSegments.map((segment, index) => (
            <line
              className="board__line"
              {...segmentProps(segment)}
              key={`segment-${index}`}
            />
          ))}
          <g className="board__positions">
            {geometry.positionPoints.map((point, index) => {
              const occupant = tokenAtPosition(index);
              const selected = selectedToken?.position === index;
              const active = occupant?.side === activeSide;
              const className = [
                'board__position',
                occupant ? 'board__position--occupied' : 'board__position--empty',
                occupant ? sideClass(occupant.side) : '',
                selected ? 'board__position--selected' : '',
                active ? 'board__position--active' : '',
                victory.achieved ? 'board__position--locked' : '',
              ].filter(Boolean).join(' ');

              return (
                <g
                  className="board__position-control"
                  key={`position-${index}`}
                  onClick={() => onPositionAction(index)}
                  onKeyDown={(event) => handlePositionKeyDown(event, index)}
                  role="button"
                  tabIndex={0}
                  aria-label={positionLabel({index, occupant, selected, phase})}
                >
                  <circle
                    className={className}
                    cx={point.x}
                    cy={point.y}
                    r="30"
                  />
                  {occupant && (
                    <text
                      className="board__token-label"
                      x={point.x}
                      y={point.y}
                      aria-hidden="true"
                    >
                      {sideLabel(occupant.side).slice(0, 1).toUpperCase()}
                    </text>
                  )}
                </g>
              );
            })}
          </g>
        </g>
      </svg>
    </div>
  );
};

export const boardMetadata = {
  positionCount: BOARD_POSITION_COUNT,
};
