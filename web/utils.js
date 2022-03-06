export const plotDataframe = (columnData, data, tag) => {
  let html = "<table border='1|1'>";
  for (let i = 0; i < columnData.length; i++) {
    html += "<th>" + columnData[i] + "</th>";
  }
  // data.length => restrict to 10
  for (let i = 0; i < 5; i++) {
    html += "<tr>";
    for (let j = 0; j < data[0].length; j++) {
      html += "<td>" + data[i][j] + "</td>";
    }
    html += "</tr>";

    html += "<tr>";
  }
  for (let j = 0; j < data[0].length; j++) {
    html += "<td>" + "..." + "</td>";
  }
  html += "</tr>";
  html += "</table>";
  document.getElementById(tag).innerHTML = html;
};

export const plotSeries = (header, data, tag) => {
  let html = "<table border='1|1'>";
  html += "<th>" + header + "</th>";
  // data.lenght => restrict to 5
  for (let i = 0; i < 5; i++) {
    html += "<tr>";

    html += "<td>" + data[i] + "</td>";

    html += "</tr>";
  }
  html += "<tr><td>...</td></tr>";
  html += "</table>";
  document.getElementById(tag).innerHTML = html;
};

export const label = (time, type, size, tag) => {
  let html =
    "<h3>Time taken to construct " + type + " (" + size + " elements) ";

  html +=
    "<strong><i><u id=`dfTime`>" + time + "</u> milliseconds</i></strong>";

  html += "</h3>";
  document.getElementById(tag).innerHTML = html;
};
