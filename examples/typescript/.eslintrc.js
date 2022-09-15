module.exports = {
  plugins: ["unused-imports"],
  rules: {
    "@typescript-eslint/consistent-type-assertions": "off",
    "@typescript-eslint/explicit-member-accessibility": "off",
    "@typescript-eslint/member-ordering": "off",
    "arrow-body-style": ["error", "as-needed"],
    "max-params": "off",
    "no-undef": "off",
    "unused-imports/no-unused-imports": "error",
    "@typescript-eslint/consistent-type-definitions": "off",
    "@typescript-eslint/method-signature-style": "off",
  },
  extends: ["alloy", "alloy/typescript"],
};
