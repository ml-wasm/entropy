const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: "wasml/entropy",
  tagline: "Dataframes powered by WebAssembly",
  url: "https://github.com/ml-wasm",
  baseUrl: "/entropy/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/wasml.png",
  organizationName: "ml-wasm",
  projectName: "entropy",
  trailingSlash: false,
  themeConfig: {
    navbar: {
      title: "wasml/entropy",
      logo: {
        alt: "My Site Logo",
        src: "img/wasml.png",
      },
      items: [
        {
          type: "doc",
          docId: "index",
          position: "left",
          label: "Documentation",
        },
        {
          href: "https://github.com/wasml/entropy",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          // editUrl: "https://github.com/wasml/entropy/edit/main/docs/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
