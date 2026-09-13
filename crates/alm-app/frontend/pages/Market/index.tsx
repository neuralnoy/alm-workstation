import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import CurveChart, { CurvePoint } from "../../charts/CurveChart";
import { Activity, ToggleLeft, ToggleRight } from "lucide-react";

export default function Market() {
  const [data, setData] = useState<CurvePoint[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Toggles
  const [showZero, setShowZero] = useState(true);
  const [showForward, setShowForward] = useState(true);
  const [showDiscount, setShowDiscount] = useState(false);

  useEffect(() => {
    async function fetchData() {
      try {
        setLoading(true);
        const result = await invoke<CurvePoint[]>("get_curve_data");
        setData(result);
        setError(null);
      } catch (e: any) {
        setError(e.toString());
      } finally {
        setLoading(false);
      }
    }

    fetchData();
  }, []);

  return (
    <div className="market-container fade-in">
      <div className="header-flex">
        <div>
          <h2>Market Data Center</h2>
          <p className="text-muted">Interactive Yield Curves & Bootstrapping Analytics</p>
        </div>
        <div className="status-badge">
          <Activity size={16} /> Live Bootstrapping
        </div>
      </div>

      <div className="glass-card mt-4">
        <div className="card-header-flex">
          <h3>USD-OIS Yield Curve (10Y)</h3>
          
          <div className="toggles">
             <div className="toggle-item" onClick={() => setShowZero(!showZero)}>
                {showZero ? <ToggleRight color="#58a6ff" /> : <ToggleLeft color="var(--text-muted)" />}
                <span style={{ color: showZero ? "#58a6ff" : "var(--text-muted)"}}>Zero Rate</span>
             </div>
             <div className="toggle-item" onClick={() => setShowForward(!showForward)}>
                {showForward ? <ToggleRight color="#d2a8ff" /> : <ToggleLeft color="var(--text-muted)" />}
                <span style={{ color: showForward ? "#d2a8ff" : "var(--text-muted)"}}>Forward Rate</span>
             </div>
             <div className="toggle-item" onClick={() => setShowDiscount(!showDiscount)}>
                {showDiscount ? <ToggleRight color="#3fb950" /> : <ToggleLeft color="var(--text-muted)" />}
                <span style={{ color: showDiscount ? "#3fb950" : "var(--text-muted)"}}>Discount Factor</span>
             </div>
          </div>
        </div>

        <div className="chart-wrapper mt-4">
          {loading ? (
            <div className="loading-state">Bootstrapping curve...</div>
          ) : error ? (
            <div className="error-state">Failed to build curve: {error}</div>
          ) : (
            <CurveChart 
              data={data} 
              showZero={showZero} 
              showForward={showForward} 
              showDiscount={showDiscount} 
            />
          )}
        </div>
      </div>
      
      <div className="info-grid mt-4">
          <div className="glass-card small-card">
              <h4>Instruments</h4>
              <ul className="muted-list">
                  <li>USD 1M Cash (4.50%)</li>
                  <li>USD 3M FRA (4.70%)</li>
                  <li>USD 1Y OIS (5.00%)</li>
                  <li>USD 5Y OIS (5.50%)</li>
                  <li>USD 10Y OIS (5.80%)</li>
              </ul>
          </div>
          <div className="glass-card small-card">
              <h4>Interpolation</h4>
              <p className="text-muted mt-2">
                 The curve is constructed using <strong>Monotone Convex</strong> interpolation (Hagan-West style monotonic cubic splines) to ensure smooth, arbitrage-free forward rates.
              </p>
          </div>
      </div>
    </div>
  );
}
