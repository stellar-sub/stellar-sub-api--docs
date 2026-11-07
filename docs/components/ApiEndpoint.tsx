import React from "react";

interface ApiEndpointProps {
  method: "GET" | "POST" | "PUT" | "DELETE";
  path: string;
  description: string;
}

export default function ApiEndpoint({ method, path, description }: ApiEndpointProps) {
  const methodColors = {
    GET: "bg-green-500",
    POST: "bg-blue-500",
    PUT: "bg-yellow-500",
    DELETE: "bg-red-500",
  };

  return (
    <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 my-4">
      <div className="flex items-center gap-3 mb-2">
        <span
          className={`${methodColors[method]} text-white text-xs font-bold px-3 py-1 rounded`}
        >
          {method}
        </span>
        <code className="text-lg font-mono">{path}</code>
      </div>
      <p className="text-gray-600 dark:text-gray-400">{description}</p>
    </div>
  );
}
