import React from "react";

interface CodeBlockProps {
  children: React.ReactNode;
  language?: string;
}

export default function CodeBlock({ children, language = "typescript" }: CodeBlockProps) {
  return (
    <div className="relative">
      {language && (
        <div className="absolute top-0 right-0 bg-gray-700 text-gray-300 text-xs px-3 py-1 rounded-bl-md">
          {language}
        </div>
      )}
      <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto">
        <code>{children}</code>
      </pre>
    </div>
  );
}
