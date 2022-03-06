import { plotSeries, plotDataframe, label } from "./utils.js";
import init, { SeriesI32, DataFrame } from "../pkg/entropy.js";

(async () => {
  await init();

  console.group(
    "%cSeries",
    "color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px"
  );
  seriesDemo();
  console.groupEnd();

  console.group(
    "%cDataFrame",
    "color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px"
  );
  dataframeDemo();
  console.groupEnd();
})();

const sizeArray = [1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000];
// const sizeArray = [1000, 5000, 10000, 50000, 100000, 500000];

const dataframeDemo = async () => {
  console.group("Dataframe");
  let file = await fetch("../data/erupt.csv");

  for (let i = 0; i < sizeArray.length; i++) {
    let sa = SeriesI32.newWithSimpleFunc(
      "Apple",
      sizeArray[i],
      (max = 10, min = 2) => Math.random() * (max - min) + min
    );

    let sb = SeriesI32.newWithSimpleFunc(
      "Banana",
      sizeArray[i],
      (max = 10, min = 2) => Math.random() * (max - min) + min
    );

    let so = SeriesI32.newWithSimpleFunc(
      "Orange",
      sizeArray[i],
      (max = 10, min = 2) => Math.random() * (max - min) + min
    );

    let sm = SeriesI32.newWithSimpleFunc(
      "Mango",
      sizeArray[i],
      (max = 10, min = 2) => Math.random() * (max - min) + min
    );

    let startTime = performance.now();
    let df = new DataFrame([
      sa.toJson(),
      sb.toJson(),
      so.toJson(),
      sm.toJson(),
    ]);
    let endTime = performance.now();
    label(
      endTime - startTime,
      "dataframe",
      parseInt(sizeArray[i]),
      "dfTime" + (i + 1)
    );
    plotDataframe(df.columns(), df.toArrays, "dataframe" + (i + 1));
  }

  console.groupEnd();
};

const seriesDemo = () => {
  const performances = [];
  console.group("SeriesI32");

  for (let i = 0; i < sizeArray.length; i++) {
    let startTime = performance.now();
    let snwf = SeriesI32.newWithSimpleFunc(
      sizeArray[i].toString(),
      sizeArray[i],
      (max = 10, min = 2) => Math.random() * (max - min) + min
    );
    let endTime = performance.now();
    label(
      endTime - startTime,
      "series",
      parseInt(sizeArray[i]),
      "sTime" + (i + 1)
    );
    plotSeries(snwf.name(), snwf.data(), "series" + (i + 1));
    performances.push(endTime - startTime);

    // let danfoStartTime = performance.now();
    // let obj_data = { Apple: snwf.data() };
    // let dfdanfo = new dfd.Series(obj_data);
    // dfdanfo.print();
    // let danfoEndTime = performance.now();

    // console.log(
    //   `Call to doSomething took ${danfoEndTime - danfoStartTime} milliseconds`
    // );
  }
  console.log("Series performance array", performances);
  console.groupEnd();
};

export default sizeArray;
