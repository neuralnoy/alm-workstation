import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer
} from "recharts";
import "./index.css";

interface Cashflow {
  date: string;
  amount: number;
  cashflow_type: "Principal" | "Interest" | "Fee" | "Total";
}

interface BondMetrics {
  cashflows: Cashflow[];
  present_value: number;
  macaulay_duration: number;
  modified_duration: number;
}

export default function Valuation() {
  const [metrics, setMetrics] = useState<BondMetrics | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  // Form State
  const [instrumentType, setInstrumentType] = useState<"Bond" | "Mortgage">("Bond");
  const [principal, setPrincipal] = useState(100000.0);
  const [couponRate, setCouponRate] = useState(0.05);
  const [startDate, setStartDate] = useState("2026-01-01");
  const [maturityDate, setMaturityDate] = useState("2036-01-01");
  const [discountRate, setDiscountRate] = useState(0.04);

  useEffect(() => {
    async function fetchMetrics() {
      setLoading(true);
      setError("");
      try {
        const response = await invoke<BondMetrics>("calculate_instrument_metrics", {
          instrumentType,
          principal,
          couponRate,
          startDate,
          maturityDate,
          discountRate,
        });
        setMetrics(response);
      } catch (e: any) {
        setError(e.toString());
      } finally {
        setLoading(false);
      }
    }
    fetchMetrics();
  }, [instrumentType, principal, couponRate, startDate, maturityDate, discountRate]);

  // Aggregate cashflows by date for the chart
  const chartData = useMemo(() => {
    if (!metrics) return [];
    const grouped = metrics.cashflows.reduce((acc: any, cf) => {
      if (!acc[cf.date]) {
        acc[cf.date] = { date: cf.date, Principal: 0, Interest: 0, Total: 0 };
      }
      if (cf.cashflow_type === "Principal") acc[cf.date].Principal += cf.amount;
      if (cf.cashflow_type === "Interest") acc[cf.date].Interest += cf.amount;
      if (cf.cashflow_type === "Total") acc[cf.date].Total += cf.amount;
      return acc;
    }, {});
    return Object.values(grouped);
  }, [metrics]);

  return (
    <div className="valuation-dashboard">
      <div className="sidebar glass-panel">
        <h2 className="sidebar-title">Instrument Setup</h2>
        
        <div className="form-group">
          <label>Type</label>
          <select 
            value={instrumentType} 
            onChange={(e) => setInstrumentType(e.target.value as "Bond" | "Mortgage")}
          >
            <option value="Bond">Fixed Rate Bond</option>
            <option value="Mortgage">Mortgage (Annuity)</option>
          </select>
        </div>

        <div className="form-group">
          <label>Principal ($)</label>
          <input 
            type="number" 
            value={principal} 
            onChange={(e) => setPrincipal(Number(e.target.value))} 
            step="1000"
          />
        </div>

        <div className="form-group">
          <label>Coupon Rate</label>
          <input 
            type="number" 
            value={couponRate} 
            onChange={(e) => setCouponRate(Number(e.target.value))} 
            step="0.005"
          />
        </div>

        <div className="form-group">
          <label>Start Date</label>
          <input 
            type="date" 
            value={startDate} 
            onChange={(e) => setStartDate(e.target.value)} 
          />
        </div>

        <div className="form-group">
          <label>Maturity Date</label>
          <input 
            type="date" 
            value={maturityDate} 
            onChange={(e) => setMaturityDate(e.target.value)} 
          />
        </div>

        <div className="form-group">
          <label>Discount Rate</label>
          <input 
            type="number" 
            value={discountRate} 
            onChange={(e) => setDiscountRate(Number(e.target.value))} 
            step="0.005"
          />
        </div>
      </div>

      <div className="main-content">
        <div className="metrics-grid">
          <div className="metric-card glass-panel">
            <h3>Present Value (PV)</h3>
            <div className="metric-value pv-value">
              {error ? "---" : metrics ? `$${metrics.present_value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}` : "Loading..."}
            </div>
          </div>
          <div className="metric-card glass-panel">
            <h3>Macaulay Duration</h3>
            <div className="metric-value dur-value">
              {error ? "---" : metrics ? `${metrics.macaulay_duration.toFixed(3)} yrs` : "Loading..."}
            </div>
          </div>
          <div className="metric-card glass-panel">
            <h3>Modified Duration</h3>
            <div className="metric-value mod-value">
              {error ? "---" : metrics ? `${metrics.modified_duration.toFixed(3)} yrs` : "Loading..."}
            </div>
          </div>
        </div>

        {error && <div className="error-banner glass-panel">Error: {error}</div>}

        {!error && metrics && (
          <>
            <div className="chart-container glass-panel">
              <h3>Cashflow Projection</h3>
              <div style={{ width: '100%', height: 350 }}>
                <ResponsiveContainer>
                  <BarChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#2d3748" vertical={false} />
                    <XAxis dataKey="date" stroke="#a0aec0" tick={{fill: '#a0aec0', fontSize: 12}} />
                    <YAxis stroke="#a0aec0" tick={{fill: '#a0aec0', fontSize: 12}} tickFormatter={(val) => `$${(val/1000)}k`} />
                    <Tooltip 
                      contentStyle={{ backgroundColor: 'rgba(17, 24, 39, 0.9)', borderColor: '#374151', borderRadius: '8px', color: '#fff' }}
                      itemStyle={{ color: '#fff' }}
                    />
                    <Legend wrapperStyle={{ paddingTop: '10px' }}/>
                    <Bar dataKey="Interest" stackId="a" fill="#10b981" radius={[0, 0, 4, 4]} />
                    <Bar dataKey="Principal" stackId="a" fill="#3b82f6" radius={instrumentType === "Mortgage" ? [4, 4, 0, 0] : [0, 0, 0, 0]} />
                    <Bar dataKey="Total" stackId="a" fill="#8b5cf6" radius={[4, 4, 0, 0]} />
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>

            <div className="cashflow-table-container glass-panel">
              <h3>Raw Schedule</h3>
              <table className="cashflow-table">
                <thead>
                  <tr>
                    <th>Date</th>
                    <th>Type</th>
                    <th className="align-right">Amount ($)</th>
                  </tr>
                </thead>
                <tbody>
                  {metrics.cashflows.map((cf, index) => (
                    <tr key={index}>
                      <td>{cf.date}</td>
                      <td>
                        <span className={`type-badge type-${cf.cashflow_type.toLowerCase()}`}>
                          {cf.cashflow_type}
                        </span>
                      </td>
                      <td className="align-right">{cf.amount.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
