import { useState } from "react";
import "./index.css";

const MENU_ITEMS = [
  "Dashboard",
  "Market",
  "Balance Sheet",
  "Instruments",
  "Valuation",
  "Scenarios",
  "Risk",
  "Liquidity",
  "FTP",
  "Reports"
];

import Valuation from "./pages/Valuation";
import Market from "./pages/Market";

function App() {
  const [activeMenu, setActiveMenu] = useState("Dashboard");

  return (
    <>
      <div className="sidebar">
        <h2>ALM Workstation</h2>
        <div className="sidebar-nav">
          {MENU_ITEMS.map(item => (
            <div 
              key={item} 
              className={`nav-item ${activeMenu === item ? 'active' : ''}`}
              onClick={() => setActiveMenu(item)}
            >
              {item}
            </div>
          ))}
        </div>
      </div>
      <div className="main-content">
        <h1>{activeMenu}</h1>
        {activeMenu === "Valuation" ? (
          <Valuation />
        ) : activeMenu === "Market" ? (
          <Market />
        ) : (
          <div className="dashboard-card">
            <p>This is the {activeMenu} view. Select an item from the sidebar to navigate the ALM modules.</p>
          </div>
        )}
      </div>
    </>
  );
}

export default App;
