import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';

export interface CurvePoint {
  date: string;
  time_years: number;
  zero_rate: number;
  forward_rate: number;
  discount_factor: number;
}

interface CurveChartProps {
  data: CurvePoint[];
  showZero: boolean;
  showForward: boolean;
  showDiscount: boolean;
}

const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    return (
      <div className="chart-tooltip">
        <p className="tooltip-date">{label}</p>
        {payload.map((entry: any, index: number) => {
            const isPercent = entry.name.includes("Rate");
            const val = isPercent ? (entry.value * 100).toFixed(3) + "%" : entry.value.toFixed(4);
            return (
                <p key={index} style={{ color: entry.color }} className="tooltip-item">
                {entry.name}: {val}
                </p>
            );
        })}
      </div>
    );
  }

  return null;
};

export default function CurveChart({ data, showZero, showForward, showDiscount }: CurveChartProps) {
  return (
    <div style={{ width: '100%', height: 400 }}>
      <ResponsiveContainer>
        <LineChart data={data} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" vertical={false} />
          
          <XAxis 
            dataKey="date" 
            stroke="var(--text-muted)" 
            tick={{ fill: 'var(--text-muted)' }}
            tickFormatter={(tick) => tick.substring(0, 7)} // Show YYYY-MM
            minTickGap={30}
          />
          
          <YAxis 
            yAxisId="left"
            stroke="var(--text-muted)" 
            tick={{ fill: 'var(--text-muted)' }}
            tickFormatter={(val) => (val * 100).toFixed(1) + "%"}
            domain={['auto', 'auto']}
          />

          <YAxis 
            yAxisId="right"
            orientation="right"
            stroke="var(--text-muted)" 
            tick={{ fill: 'var(--text-muted)' }}
            tickFormatter={(val) => val.toFixed(2)}
            domain={[0, 1.05]}
          />

          <Tooltip content={<CustomTooltip />} />
          <Legend wrapperStyle={{ paddingTop: '20px' }} />

          {showZero && (
            <Line 
              yAxisId="left"
              type="monotone" 
              name="Zero Rate (Cont.)"
              dataKey="zero_rate" 
              stroke="#58a6ff" 
              strokeWidth={3}
              dot={false}
              activeDot={{ r: 6, fill: "#58a6ff" }}
            />
          )}

          {showForward && (
             <Line 
             yAxisId="left"
             type="stepAfter" 
             name="3M Fwd Rate"
             dataKey="forward_rate" 
             stroke="#d2a8ff" 
             strokeWidth={2}
             strokeDasharray="5 5"
             dot={false}
           />
          )}

          {showDiscount && (
             <Line 
             yAxisId="right"
             type="monotone" 
             name="Discount Factor"
             dataKey="discount_factor" 
             stroke="#3fb950" 
             strokeWidth={2}
             dot={false}
           />
          )}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
