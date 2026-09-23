import { useState } from "react";
import "./index.css";

import Valuation from "./pages/Valuation";
import DataHub from "./pages/DataHub";

const WORKSPACE_MODULES = [
  "Dashboard",
  "Valuation",
  "Scenarios",
  "Risk",
  "Liquidity",
  "FTP",
  "Reports"
];

function App() {
  const [activeNav, setActiveNav] = useState("Dashboard");

  return (
    <>
      <div className="top-nav">
        <div className="brand">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="28" height="28">
            <defs>
              <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style={{stopColor:'#58a6ff', stopOpacity:1}} />
                <stop offset="100%" style={{stopColor:'#3fb950', stopOpacity:1}} />
              </linearGradient>
            </defs>
            <polygon points="100,30 40,160 80,160 100,110 120,160 160,160" fill="url(#grad1)" />
            <polygon points="100,60 70,130 130,130" fill="#0d1117" />
            <circle cx="100" cy="115" r="10" fill="url(#grad1)" />
          </svg>
          ALM Workstation
        </div>
        <div className="top-nav-links">
          <div className="dropdown">
            <div className={`top-nav-item ${WORKSPACE_MODULES.includes(activeNav) ? 'active' : ''}`}>
              Modules ▾
            </div>
            <div className="dropdown-content">
              {WORKSPACE_MODULES.map(item => (
                <div 
                  key={item} 
                  className={`dropdown-item ${activeNav === item ? 'active' : ''}`}
                  onClick={() => setActiveNav(item)}
                >
                  <span style={{ fontSize: '1.2rem', opacity: 0.7, marginRight: '8px' }}>❖</span> {item}
                </div>
              ))}
            </div>
          </div>
          
          <div 
            className={`top-nav-item ${activeNav === 'Data Hub' ? 'active' : ''}`}
            onClick={() => setActiveNav('Data Hub')}
          >
            Data Hub
          </div>
          <div 
            className={`top-nav-item ${activeNav === 'Settings' ? 'active' : ''}`}
            onClick={() => setActiveNav('Settings')}
          >
            Settings
          </div>
        </div>
      </div>
      <div className="app-body">
        <div className="main-content">
          {activeNav === "Data Hub" ? (
            <DataHub />
          ) : activeNav === "Settings" ? (
            <div className="glass-card fade-in">
              <h3>Settings</h3>
              <p className="text-muted mt-4">Application settings and configuration.</p>
            </div>
          ) : (
            <>
              <h1 className="fade-in">{activeNav}</h1>
              {activeNav === "Valuation" ? (
                <div className="fade-in"><Valuation /></div>
              ) : (
                <div className="glass-card fade-in">
                  <div className="card-header-flex">
                    <h3>{activeNav} Overview</h3>
                  </div>
                  <p className="text-muted mt-4">
                    This is the {activeNav} view. Select an item from the modules dropdown to navigate.
                  </p>
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </>
  );
}

export default App;
