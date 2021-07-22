import init, {
  SeriesF64,
  SeriesI32,
  DataFrame,
  ColumnType,
  SeriesSTR,
  readcsv,
} from "../pkg/entropy.js";

(async () => {
  await init();

  // console.group(
  //   "%cSeries",
  //   "color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px"
  // );
  // seriesDemo();
  // console.groupEnd();

  // console.group(
  //   "%cDataFrame",
  //   "color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px"
  // );
  dataframeDemo();
  // console.groupEnd();
})();

// function hello() {
//   alert("hello");
// }

// window.hello = hello;

const dataframeDemo = async () => {
  // const [fileHandle] = await window.showOpenFilePicker();
  // const file = await fileHandle.getFile();

  let file = await fetch("../data/hwi.csv");

  // console.group("DataFrame");

  // let sera = new SeriesI32("Apples", [1, 2, 3, 4]);
  // let sero = new SeriesF64("Orange", [1.1, 2.1, 3.1, 4.1]);
  // let serb = new SeriesSTR("Banana", ["ban", "nan", "qs", "rty"]);

  // console.log("DataFrame");
  // let df = new DataFrame([sera.toJson(), sero.toJson(), serb.toJson()]);
  // console.table(df.displayTable);

  // console.log(await readcsv(file));

  let df = await readcsv(file);
  console.log(df.columns());
  console.table(df.displayTable);
  // console.log(df.columns());
  // console.log(df.dataTypes());
  console.log("DataFrame size");
  console.log(df.rowsCount());
  console.log(df.columnsCount());

  // console.log("DataFrame show Columns");
  // console.log(df.columns());

  // console.log("DataFrame show Datatypes");
  console.log(df.dataTypes());

  // console.log("DataFrame append series");
  // let sertest = new SeriesI32("Fruits", [9, 8, 9, 8]);
  // df.append(ColumnType.INTEGER, sertest.toJson());
  // console.log(df.display);

  // console.log("DataFrame loc get series with col name");
  // console.log(df.loc("Banana"));

  // console.log("DataFrame ilor get row");
  // console.log(df.ilocr(2));

  // console.log("DataFrame iloc get col");
  // console.log(df.ilocc(1));

  // console.group("DataFrame Math operations");
  // // console.log("DataFrame min");
  // console.log(df.min("Apples"));
  // console.log(df.maxColumns());
  // console.log(df.meanColumns());
  // console.groupEnd();

  // console.groupEnd();
};

window.dataframeDemo = dataframeDemo;

const seriesDemo = () => {
  console.group("SeriesI32");

  let ser1 = new SeriesI32("Integers", [1, 2, 3, 4]);
  console.log("SeriesI32");
  console.log(ser1.display);

  console.log("SeriesI32 length");
  console.log(ser1.len());

  console.log("SeriesI32 get at index 2 (0 based)");
  console.log(ser1.get(2));

  console.log("SeriesI32 set at index (0) value (11)");
  ser1.set(0, 11);
  console.log(ser1.display);

  console.log("SeriesI32 swap values between index (0) and index (1)");
  ser1.swap(0, 1);
  console.log(ser1.display);

  console.log("SeriesI32 reverse (without return)");
  ser1.reverse();
  console.log(ser1.display);

  console.log("SeriesI32 reversed (with return)");
  console.log(ser1.reversed().data);

  console.log("SeriesI32 append (without return)");
  ser1.append(22);
  console.log(ser1.display);

  console.log("SeriesI32 appended (with return)");
  console.log(ser1.appended(33).data);

  console.log("SeriesI32 extend (extend data in series) (without return)");
  ser1.extend([44, 44]);
  console.log(ser1.display);

  console.log("SeriesI32 extended (extend data in series) (with return)");
  console.log(ser1.extended([55, 55]).data);

  console.log(
    "SeriesI32 insert at index put value push rest by one (without return)"
  );
  ser1.insert(0, 55);
  console.log(ser1.display);

  console.log(
    "SeriesI32 inserted at index put value push rest by one (with return)"
  );
  console.log(ser1.inserted(0, 11).data);

  console.log(
    "SeriesI32 splice remove the value at the specified index and return the result"
  );
  console.log(ser1.splice(5));

  console.log(
    "SeriesI32 splice remove the value at the specified index and return the array"
  );
  console.log(ser1.spliced(6)[0].data);

  console.group("SeriesI32 Math Operations");
  console.log("SeriesI32 sum");
  console.log(ser1.sum());

  console.log("SeriesI32 product");
  console.log(ser1.product());

  console.log("SeriesI32 mean");
  console.log(ser1.mean());

  console.log("SeriesI32 median");
  console.log(ser1.median());

  console.log("SeriesI32 max");
  console.log(ser1.max());

  console.log("SeriesI32 min");
  console.log(ser1.min());

  console.groupEnd();

  console.groupEnd();

  console.group("SeriesF64");
  let serf1 = new SeriesF64("Floating", [1.1, 2.2, 3.3, 4.4]);
  console.log("SeriesF64");
  console.log(serf1.display);

  console.log("SeriesF64 length");
  console.log(serf1.len());

  console.log("SeriesF64 get at index 2 (0 based)");
  console.log(serf1.get(2));

  console.log("SeriesF64 set at index (0) value (11.11)");
  serf1.set(0, 11.11);
  console.log(serf1.display);

  console.log("SeriesF64 swap values between index (0) and index (1)");
  serf1.swap(0, 1);
  console.log(serf1.display);

  console.log("SeriesF64 reverse (without return)");
  serf1.reverse();
  console.log(serf1.display);

  console.log("SeriesF64 reversed (with return)");
  console.log(serf1.reversed().data);

  console.log("SeriesF64 append (without return)");
  serf1.append(22.2);
  console.log(serf1.display);

  console.log("SeriesF64 appended (with return)");
  console.log(serf1.appended(33.3).data);

  console.log("SeriesF64 extend (extend data in series) (without return)");
  serf1.extend([44.4, 44.4]);
  console.log(serf1.display);

  console.log("SeriesF64 extended (extend data in series) (with return)");
  console.log(serf1.extended([55, 55]).data);

  console.log(
    "SeriesF64 insert at index put value push rest by one (without return)"
  );
  serf1.insert(0, 55.5);
  console.log(serf1.display);

  console.log(
    "SeriesF64 inserted at index put value push rest by one (with return)"
  );
  console.log(serf1.inserted(0, 11.1).data);

  console.log(
    "SeriesF64 splice remove the value at the specified index and return the result"
  );
  console.log(serf1.splice(5));

  console.log(
    "SeriesF64 splice remove the value at the specified index and return the array"
  );
  console.log(serf1.spliced(6)[0].data);

  console.group("SeriesF64 Math Operations");
  console.log("SeriesF64 sum");
  console.log(serf1.sum());

  console.log("SeriesF64 product");
  console.log(serf1.product());

  console.log("SeriesF64 mean");
  console.log(serf1.mean());

  console.log("SeriesF64 median");
  console.log(serf1.median());

  console.log("SeriesF64 max");
  console.log(serf1.max());

  console.log("SeriesF64 min");
  console.log(serf1.min());

  console.groupEnd();

  console.groupEnd();

  console.group("SeriesSTR");
  let sers1 = new SeriesSTR("StringSTR", ["abc", "cbe", "rty", "qwe"]);
  console.log("SeriesSTR");
  console.log(sers1.display);

  console.log("SeriesSTR length");
  console.log(sers1.len());

  console.log("SeriesSTR get at index 2 (0 based)");
  console.log(sers1.get(2));

  console.log("SeriesSTR set at index (0) value (yoy)");
  sers1.set(0, "yoy");
  console.log(sers1.display);

  console.log("SeriesSTR swap values between index (0) and index (1)");
  sers1.swap(0, 1);
  console.log(sers1.display);

  console.log("SeriesSTR reverse (without return)");
  sers1.reverse();
  console.log(sers1.display);

  console.log("SeriesSTR reversed (with return)");
  console.log(sers1.reversed().data);

  console.log("SeriesSTR append (without return)");
  sers1.append("opo");
  console.log(sers1.display);

  console.log("SeriesSTR appended (with return)");
  console.log(sers1.appended("vov").data);

  console.log("SeriesSTR extend (extend data in series) (without return)");
  sers1.extend(["yui", "iop"]);
  console.log(sers1.display);

  console.log("SeriesSTR extended (extend data in series) (with return)");
  console.log(sers1.extended(["cvb", "bnm"]).data);

  console.log(
    "SeriesSTR insert at index put value push rest by one (without return)"
  );
  sers1.insert(0, "zxc");
  console.log(sers1.display);

  console.log(
    "SeriesSTR inserted at index put value push rest by one (with return)"
  );
  console.log(sers1.inserted(0, "fgh").data);

  console.log(
    "SeriesSTR splice remove the value at the specified index and return the result"
  );
  console.log(sers1.splice(5));

  console.log(
    "SeriesSTR splice remove the value at the specified index and return the array"
  );
  console.log(sers1.spliced(6)[0].data);

  console.groupEnd();
};
