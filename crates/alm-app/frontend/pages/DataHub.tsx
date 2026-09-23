import { useState } from "react";
import Market from "./Market";

const DATA_MODULES = [
  "Market Data",
  "Balance Sheet",
  "Instruments"
];

export default function DataHub() {
  const [activeTab, setActiveTab] = useState("Market Data");

  return (
    <div className="glass-card fade-in" style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <div className="card-header-flex">
        <h3>Data Management Hub</h3>
        <div className="toggles">
          {DATA_MODULES.map(tab => (
            <div 
              key={tab}
              className="toggle-item"
              style={{
                color: activeTab === tab ? 'var(--accent)' : 'var(--text-muted)',
                borderBottom: activeTab === tab ? '2px solid var(--accent)' : 'none',
                paddingBottom: '4px'
              }}
              onClick={() => setActiveTab(tab)}
            >
              {tab}
            </div>
          ))}
        </div>
      </div>
      
      <div className="mt-4" style={{ flex: 1, overflowY: 'auto' }}>
        {activeTab === "Market Data" ? (
          <Market />
        ) : activeTab === "Balance Sheet" ? (
          <div className="info-grid">
            <div className="small-card glass-card">
              <h4>Portfolios</h4>
              <p className="text-muted">Manage bank portfolios, accounts, and hierarchies.</p>
            </div>
            <div className="small-card glass-card">
              <h4>Positions</h4>
              <p className="text-muted">Current balances and historical positions.</p>
            </div>
          </div>
        ) : (
          <div className="info-grid">
            <div className="small-card glass-card">
              <h4>Instrument Types</h4>
              <p className="text-muted">Configure instrument definitions and attributes.</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
