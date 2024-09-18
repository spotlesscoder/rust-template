import { UserConfig } from "@commitlint/types";

const Configuration: UserConfig = {
    extends: ["@commitlint/config-conventional"],
    rules: {
        "header-max-length": [2, "always", 72],
        "subject-case": [0], // Disable subject casing rule
    },
};

export default Configuration;
