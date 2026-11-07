import React from "react";

interface CalloutBoxProps {
  type?: "info" | "warning" | "danger" | "success";
  children: React.ReactNode;
}

export default function CalloutBox({ type = "info", children }: CalloutBoxProps) {
  const styles = {
    info: "bg-blue-50 dark:bg-blue-900/20 border-blue-500 text-blue-900 dark:text-blue-100",
    warning: "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-500 text-yellow-900 dark:text-yellow-100",
    danger: "bg-red-50 dark:bg-red-900/20 border-red-500 text-red-900 dark:text-red-100",
    success: "bg-green-50 dark:bg-green-900/20 border-green-500 text-green-900 dark:text-green-100",
  };

  const icons = {
    info: "ℹ️",
    warning: "⚠️",
    danger: "🛑",
    success: "✅",
  };

  return (
    <div className={`border-l-4 p-4 my-4 rounded-r-lg ${styles[type]}`}>
      <div className="flex items-start">
        <span className="text-2xl mr-3">{icons[type]}</span>
        <div className="flex-1">{children}</div>
      </div>
    </div>
  );
}
