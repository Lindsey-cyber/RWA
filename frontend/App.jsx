import React, { useState } from "react";
import InvestorPanel from "./components/InvestorPanel.jsx";
import AdminPanel from "./components/AdminPanel.jsx";

export default function App() {
  const [isAdmin, setIsAdmin] = useState(false);

  return (
    <div className="p-8 flex flex-col gap-6 items-center">
      <h1 className="text-2xl font-bold">Tranche 测试前端</h1>
      <div className="flex gap-4">
        <button
          className={`px-4 py-2 rounded ${!isAdmin ? "bg-blue-600 text-white" : "bg-gray-300"}`}
          onClick={() => setIsAdmin(false)}
        >
          投资者页面
        </button>
        <button
          className={`px-4 py-2 rounded ${isAdmin ? "bg-blue-600 text-white" : "bg-gray-300"}`}
          onClick={() => setIsAdmin(true)}
        >
          管理员页面
        </button>
      </div>

      {!isAdmin ? <InvestorPanel /> : <AdminPanel />}
    </div>
  );
}
