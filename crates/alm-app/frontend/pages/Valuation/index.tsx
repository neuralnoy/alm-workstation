import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./index.css";

interface Cashflow {
  date: string;
  amount: number;
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

  useEffect(() => {
    async function fetchMetrics() {
      try {
        const response = await invoke<BondMetrics>("calculate_bond_metrics", {
          principal: 1000.0,
          couponRate: 0.05,
          startDate: "2026-01-01",
          maturityDate: "2031-01-01",
          discountRate: 0.04
        });
        setMetrics(response);
      } catch (e: any) {
        setError(e.toString());
      } finally {
        setLoading(false);
      }
    }
    fetchMetrics();
  }, []);

  if (loading) return <div>Calculating bond metrics...</div>;
  if (error) return <div style={{ color: "red" }}>Error: {error}</div>;
  if (!metrics) return <div>No data.</div>;

  return (
    <div className="valuation-dashboard">
      <div className="metrics-grid">
        <div className="metric-card">
          <h3>Present Value (PV)</h3>
          <div className="metric-value">${metrics.present_value.toFixed(2)}</div>
        </div>
        <div className="metric-card">
          <h3>Macaulay Duration</h3>
          <div className="metric-value">{metrics.macaulay_duration.toFixed(3)} yrs</div>
        </div>
        <div className="metric-card">
          <h3>Modified Duration</h3>
          <div className="metric-value">{metrics.modified_duration.toFixed(3)} yrs</div>
        </div>
      </div>

      <div className="cashflow-table-container">
        <h3>Cashflow Schedule</h3>
        <table className="cashflow-table">
          <thead>
            <tr>
              <th>Period</th>
              <th>Date</th>
              <th>Amount ($)</th>
            </tr>
          </thead>
          <tbody>
            {metrics.cashflows.map((cf, index) => (
              <tr key={index}>
                <td>{index + 1}</td>
                <td>{cf.date}</td>
                <td>{cf.amount.toFixed(2)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
