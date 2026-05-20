/** @type {import('next').NextConfig} */
const nextConfig = {
  // Static export — Tauri bundles the output from /out
  output: "export",
  trailingSlash: true,

  // Images don't need a loader in static-export mode
  images: {
    unoptimized: true,
  },

  // Silence TypeScript build errors (Tauri handles the binary separately)
  typescript: {
    ignoreBuildErrors: true,
  },

  eslint: {
    ignoreDuringBuilds: true,
  },

  reactStrictMode: false,

  webpack: (config) => {
    // Tauri runs in Node-like env — stub out browser-only Node builtins
    config.resolve.fallback = {
      fs: false,
      net: false,
      tls: false,
    };
    return config;
  },
};

module.exports = nextConfig;