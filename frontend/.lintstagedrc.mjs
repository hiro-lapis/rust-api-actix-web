const config = {
  "*.{js,jsx,ts,tsx,mjs,cjs}": [
    "eslint --fix --no-warn-ignored",
    "prettier --write",
  ],
  "*.{json,css,md}": "prettier --write",
};

export default config;
