import React from "react";
import clsx from "clsx";
import styles from "./HomepageFeatures.module.css";

const FeatureList = [
  {
    title: "Series in javascript",
    Svg: require("../../static/img/undraw_docusaurus_mountain.svg").default,
    description: (
      <>
        <code>Series</code> in Entropy use{" "}
        <a target="_blank" href="https://github.com/wasml/linalg">
          wasml/linalg
        </a>{" "}
        <code>Vectors</code> implementation.
      </>
    ),
  },
  {
    title: "Dataframes in javascript",
    Svg: require("../../static/img/undraw_docusaurus_tree.svg").default,
    description: (
      <>
        <code>Dataframes</code> in Entropy utilize <code>Series</code> for
        faster easier use.
      </>
    ),
  },
  {
    title: "Powered by WebAssembly",
    Svg: require("../../static/img/undraw_docusaurus_react.svg").default,
    description: (
      <>
        Programs that are compiled into native machine code tend to be faster
        than interpreted code.
      </>
    ),
  },
];

function Feature({ Svg, title, description }) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <Svg className={styles.featureSvg} alt={title} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
