module.exports = {
  plugins: {
    "postcss-import": {},
    tailwindcss: {},
    autoprefixer: {},
    ...(process.env.TRUNK_PROFILE === "release" ? { cssnano: {} } : {}),
  },
};
