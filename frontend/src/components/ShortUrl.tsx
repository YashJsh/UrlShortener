"use client";
import { shortUrl } from "@/api/ShortUrl";
import React, { useState } from "react";
import { Loader } from 'lucide-react';

const ShortUrl = () => {
  const [url, setUrl] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<{
    originalUrl: string;
    shortUrl: string;
  } | null>(null);

  const handleShorten = async () => {
    try {
      setLoading(true);
      const data = await shortUrl(url);
      if (data) setResult(data);
    } catch (error) {
      console.error("Failed to shorten URL:", error);
    }finally{
        setLoading(false);
    }
  };

  return (
    <div className="w-screen  flex flex-col justify-center items-center gap-5">
      <div className="w-1/2 bg-gray-100 flex flex-col items-center px-2 py-4 gap-4 rounded-2xl shadow-2xl">
        <h1 className="font-semibold text-4xl uppercase">
          Enter the url to be shortened
        </h1>
        <div className="w-full flex justify-center items-center gap-3">
          <input
            type="text"
            placeholder="Enter the Url"
            className="px-2 py-4 w-full outline-none border-blue-500 border-2 rounded-xl text-gray-600 text-xl"
            onChange={(e) => {
              setUrl(e.target.value);
            }}
          />
          <button
            className="px-4 py-4 uppercase font-semibold bg-blue-600 rounded-2xl text-white hover:bg-blue-700"
            onClick={handleShorten}
            
          >
            {loading ?  <Loader className="animate-spin" /> : "Shorten Url" }
          </button>
        </div>
      </div>

      {result && (
        <div className="w-1/2 bg-gray-100 flex flex-col items-center px-2 py-4 gap-4 rounded-2xl shadow-2xl">
          <h1 className="text-xl">Shortened Url</h1>
          <input
            title="ShortUrl"
            type="text"
            value={result?.shortUrl}
            readOnly
            className="px-2 py-4 w-full outline-none border-blue-500 border-2 rounded-xl text-gray-600 text-xl"
          />
          <button
            onClick={() => {
              if (result?.shortUrl) {
                navigator.clipboard.writeText(result.shortUrl);
                alert("URL copied to clipboard!");
              }
            }}
            className="bg-blue-600 hover:bg-blue-700 uppercase text-white px-4 py-2 rounded-xl transition"
          > 
            Copy
          </button>
        </div>
      )}
    </div>
  );
};

export default ShortUrl;
