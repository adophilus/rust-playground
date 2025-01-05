import { $ } from "bun";
import { exists } from "node:fs/promises";

const BUILD_CONFIG = {
  version: 3,
  routes: [
    {
      src: "/.*",
      dest: "/",
    },
  ],
};

const FUNCTION_CONFIG = {
  runtime: "nodejs20.x",
  handler: "index.mjs",
};

const VERCEL_OUTPUT_DIRECTORY = "./.vercel/output";
const VERCEL_FUNCTION_DIRECTORY = `${VERCEL_OUTPUT_DIRECTORY}/functions/index.func`;
const BUILD_DIRECTORY = "./build";
const PUBLIC_DIRECTORY = "./public";

const writeConfigs = async () => {
  await Bun.write(
    `${VERCEL_OUTPUT_DIRECTORY}/config.json`,
    JSON.stringify(BUILD_CONFIG),
  );

  await Bun.write(
    `${VERCEL_FUNCTION_DIRECTORY}/.vc-config.json`,
    JSON.stringify(FUNCTION_CONFIG),
  );
};

const build = async () => {
  if (await exists(BUILD_DIRECTORY)) {
    await $`rm -rf ${BUILD_DIRECTORY}`;
    await $`mkdir -p ${BUILD_DIRECTORY}`;
  } else {
    await $`mkdir -p ${BUILD_DIRECTORY}`;
  }

  const buildOutput = await Bun.build({
    entrypoints: ["./scripts/server.ts"],
    outdir: BUILD_DIRECTORY,
    target: "node",
  });

  if (buildOutput.success === false) {
    console.error(buildOutput);
    throw new Error("❌ Build failed");
  }

  await $`mv ${BUILD_DIRECTORY}/server.js ${BUILD_DIRECTORY}/index.mjs`;
  await $`mkdir ${BUILD_DIRECTORY}/public`;
  await $`cp -r ${PUBLIC_DIRECTORY} ${BUILD_DIRECTORY}/public`;
};

const moveBuiltArtifacts = async () => {
  await $`cp -r ${BUILD_DIRECTORY}/* ${VERCEL_FUNCTION_DIRECTORY}`;
};

const main = async () => {
  console.log("🚀 Building...");

  if (!(await exists(VERCEL_OUTPUT_DIRECTORY))) {
    await $`mkdir -p ${VERCEL_OUTPUT_DIRECTORY}`;
  } else {
    await $`rm -rf ${VERCEL_OUTPUT_DIRECTORY}`;
    await $`mkdir -p ${VERCEL_OUTPUT_DIRECTORY}`;
  }

  if (!(await exists(VERCEL_FUNCTION_DIRECTORY))) {
    await $`mkdir -p ${VERCEL_FUNCTION_DIRECTORY}`;
  }

  await writeConfigs();

  await build();

  await moveBuiltArtifacts();

  console.log("✅ Build successful");
};

main();
