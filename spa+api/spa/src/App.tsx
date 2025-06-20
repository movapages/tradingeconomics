import React, { useEffect, useState } from "react";
import Plot from "react-plotly.js";
import { status$, refreshStatus, importNames$, exportNames$, refreshImportNames, refreshExportNames, pie$, refreshPie } from "./services/api";

/**
 * Main App component for the TradingEconomics SPA.
 * Handles sidebar status, tab selection, data fetching, and layout.
 */
export default function App() {
  /**
   * Status object from /api/status endpoint.
   */
  const [status, setStatus] = useState<any>(null);
  /**
   * Current selected tab: "import", "export", or "" (none).
   */
  const [tab, setTab] = useState<"" | "import" | "export">("");
  /**
   * Table data for the selected tab (import/export names).
   */
  const [tableData, setTableData] = useState<any[] | null>(null);
  /**
   * Pie chart data for the selected tab.
   */
  const [pieData, setPieData] = useState<any[] | null>(null);
  /**
   * Loading state for table and pie data.
   */
  const [loading, setLoading] = useState(false);
  /**
   * Error message for data fetch failures.
   */
  const [error, setError] = useState<string | null>(null);

  /**
   * Subscribe to status$ observable for live status updates.
   */
  useEffect(() => {
    const sub = status$.subscribe(setStatus);
    return () => sub.unsubscribe();
  }, []);

  /**
   * Subscribe to table and pie data based on selected tab.
   * Fetches import/export names and pie data when tab changes.
   */
  useEffect(() => {
    let tableSub: any, pieSub: any;
    setTableData(null);
    setPieData(null);
    setError(null);
    if (tab === "import") {
      setLoading(true);
      refreshImportNames();
      refreshPie();
      tableSub = importNames$.subscribe((data) => {
        if (data?.error) setError(data.error);
        setTableData(Array.isArray(data) ? data : null);
        setLoading(false);
      });
      pieSub = pie$.subscribe((data) => setPieData(Array.isArray(data) ? data : null));
    } else if (tab === "export") {
      setLoading(true);
      refreshExportNames();
      refreshPie();
      tableSub = exportNames$.subscribe((data) => {
        if (data?.error) setError(data.error);
        setTableData(Array.isArray(data) ? data : null);
        setLoading(false);
      });
      pieSub = pie$.subscribe((data) => setPieData(Array.isArray(data) ? data : null));
    }
    return () => {
      if (tableSub) tableSub.unsubscribe();
      if (pieSub) pieSub.unsubscribe();
    };
  }, [tab]);

  /**
   * Handle refresh button click: refresh status and clear working area.
   */
  const handleRefresh = () => {
    refreshStatus();
    setTab(""); // Clear working area
  };

  /**
   * Format ISO date string as a short, human-readable date/time.
   * @param d ISO date string
   * @returns Formatted date string or dash if not available
   */
  const formatDate = (d: string | undefined) =>
    d ? new Date(d).toLocaleString(undefined, { year: '2-digit', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }) : <span className="text-gray-400">-</span>;

  // Pie chart data transformation
  const pieLabels = pieData?.map((item) => item.label);
  const pieValues = pieData?.map((item) => item.count);

  return (
    <div className="flex h-screen bg-white">
      {/* Sidebar */}
      <aside className="w-64 bg-white flex flex-col border-r">
        <div className="w-full px-6 pt-8 pb-4">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Data Fetch Status</h2>
            <button
              className="ml-2 p-2 rounded-full bg-blue-500 hover:bg-blue-600 transition flex items-center justify-center"
              onClick={handleRefresh}
              title="Refresh"
            >
              {/* Reload Icon SVG */}
              <svg
                className="w-2 h-2 text-white"
                fill="none"
                stroke="currentColor"
                strokeWidth={2}
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M4 4v5h.582M20 20v-5h-.581M5.5 19A9 9 0 1 0 6 6.5l1.5 1.5M18.5 5A9 9 0 0 1 18 17.5l-1.5-1.5"
                />
              </svg>
            </button>
          </div>
          <ul className="mb-2 text-gray-700 text-xs space-y-2">
            <li>
              Status:{" "}
              {status?.error ? (
                <span className="text-red-500">Error</span>
              ) : status ? (
                <span className="text-green-600">Loaded</span>
              ) : (
                <span className="text-gray-400">Loading...</span>
              )}
            </li>
            <li>
              Total Records: {status?.total_records ?? <span className="text-gray-400">-</span>}
            </li>
            <li>
              Last Updated: {formatDate(status?.last_updated)}
            </li>
            {status?.error && <li className="text-red-500">{status.error}</li>}
          </ul>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col p-8 bg-white h-screen">
        {/* Tabs */}
        <div className="flex space-x-4 mb-6">
          <button
            className={`px-6 py-2 rounded shadow text-lg font-medium ${tab === "import" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
            onClick={() => setTab("import")}
          >
            Import
          </button>
          <button
            className={`px-6 py-2 rounded shadow text-lg font-medium ${tab === "export" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
            onClick={() => setTab("export")}
          >
            Export
          </button>
        </div>

        {/* Content Area */}
        <div className="flex-1 flex flex-col bg-white rounded-lg p-8 border min-h-0">
          {/* Show nothing until a tab is selected */}
          {tab === "" ? (
            <div className="w-full h-full flex items-center justify-center text-gray-300 text-xl">Select Import or Export</div>
          ) : loading ? (
            <div className="w-full h-full flex items-center justify-center text-gray-400 text-lg">Loading...</div>
          ) : error ? (
            <div className="w-full h-full flex items-center justify-center text-red-500 text-lg">{error}</div>
          ) : (
            <>
              {/* Pie Chart at the top */}
              <div className="w-full flex items-center justify-center mb-8">
                <div className="w-96 h-64 bg-white rounded-lg shadow flex items-center justify-center border">
                  {pieData && pieLabels && pieValues && pieLabels.length > 0 && pieValues.length > 0 ? (
                    <Plot
                      data={[
                        {
                          type: "pie",
                          labels: pieLabels,
                          values: pieValues,
                          textinfo: "label+percent",
                          hoverinfo: "label+value+percent",
                          marker: {
                            colors: ["#0088FE", "#FF8042", "#00C49F", "#FFBB28", "#FF6666", "#A28CFF", "#FFB6C1", "#B0E0E6"],
                          },
                        },
                      ]}
                      layout={{
                        width: 350,
                        height: 230,
                        margin: { t: 20, b: 20, l: 20, r: 20 },
                        showlegend: true,
                      }}
                      style={{ width: "100%", height: "100%" }}
                      config={{ displayModeBar: false }}
                    />
                  ) : (
                    <span className="text-gray-400">[Pie Chart]</span>
                  )}
                </div>
              </div>
              {/* Table with scroll */}
              <div className="flex-1 min-h-0">
                <div className="h-full overflow-y-auto rounded-lg">
                  <table className="w-full border-collapse rounded-lg overflow-hidden shadow">
                    <thead>
                      <tr className="bg-green-200 text-gray-800">
                        <th className="py-2 px-4">
                          Category ({Array.isArray(tableData) ? tableData.length : 0})
                        </th>
                      </tr>
                    </thead>
                    <tbody className="text-xs">
                      {Array.isArray(tableData) && tableData.length > 0 ? (
                        tableData.map((row, idx) => (
                          <tr key={idx} className={idx % 2 === 1 ? "bg-green-50" : ""}>
                            <td className="py-2 px-4 truncate whitespace-nowrap max-w-xs">
                              <span className="font-semibold mr-2">
                                {tab === "import" ? "Import" : "Export"}
                              </span>
                              {row}
                            </td>
                          </tr>
                        ))
                      ) : (
                        <tr>
                          <td className="py-2 px-4 text-gray-400">No data</td>
                        </tr>
                      )}
                    </tbody>
                  </table>
                </div>
              </div>
            </>
          )}
        </div>
      </main>
    </div>
  );
} 