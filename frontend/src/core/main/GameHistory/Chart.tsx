import React, { ReactElement, useState } from 'react';

import {
  Sector, Pie, PieChart, ResponsiveContainer,
} from 'recharts';
import style from './GameHistory.module.scss';

interface DataPoint { type: string, data: number }

interface Props {
  data: Array<DataPoint>
}

interface ActiveProps {
  cx: number,
  cy: number,
  midAngle: number,
  innerRadius: number, outerRadius: number, startAngle: number, endAngle: number,
  fill: string, payload: DataPoint, percent: number, value: number;
}
const renderActiveShape = ({
  cx, cy, midAngle, innerRadius, outerRadius, startAngle, endAngle,
  fill, payload, percent, value,
}: ActiveProps): ReactElement => {
  const RADIAN = Math.PI / 180;
  const sin = Math.sin(-RADIAN * midAngle);
  const cos = Math.cos(-RADIAN * midAngle);
  const sx = cx + (outerRadius + 10) * cos;
  const sy = cy + (outerRadius + 10) * sin;
  const mx = cx + (outerRadius + 30) * cos;
  const my = cy + (outerRadius + 30) * sin;
  const ex = mx + (cos >= 0 ? 1 : -1) * 22;
  const ey = my;
  const textAnchor = cos >= 0 ? 'start' : 'end';

  return (
    <g>
      <text x={cx} y={cy} dy={8} textAnchor="middle" fill={fill}>{payload.type}</text>
      <Sector
        cx={cx}
        cy={cy}
        innerRadius={innerRadius}
        outerRadius={outerRadius}
        startAngle={startAngle}
        endAngle={endAngle}
        fill={fill}
      />
      <Sector
        cx={cx}
        cy={cy}
        startAngle={startAngle}
        endAngle={endAngle}
        innerRadius={outerRadius + 6}
        outerRadius={outerRadius + 10}
        fill={fill}
      />
      <path d={`M${sx},${sy}L${mx},${my}L${ex},${ey}`} stroke={fill} fill="none" />
      <circle cx={ex} cy={ey} r={2} fill={fill} stroke="none" />
      <text x={ex + (cos >= 0 ? 1 : -1) * 12} y={ey} textAnchor={textAnchor} fill="#333">{`PV ${value}`}</text>
      <text x={ex + (cos >= 0 ? 1 : -1) * 12} y={ey} dy={18} textAnchor={textAnchor} fill="#999">
        {`(Rate ${(percent * 100).toFixed(2)}%)`}
      </text>
    </g>
  );
};

export function PieChartCustom({ data }: Props): ReactElement {
  const [activeIndex, setActiveIndex] = useState(0);
  const onPieEnter = (data: any, index: number) => setActiveIndex(index);

  return (
    <div className={style.chart}>
      <ResponsiveContainer>
        <PieChart>
          <Pie
            dataKey="data"
            activeIndex={activeIndex}
            activeShape={renderActiveShape}
            data={data}
            innerRadius={100}
            // outerRadius={80}
            fill="#9672A2"
            onMouseEnter={onPieEnter}
          />
        </PieChart>
      </ResponsiveContainer>
    </div>
  );
}
