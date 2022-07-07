
Crellinor.worldmap = function(data, size) {

  var worldSize = data["worldSize"];

  var BSIZE = size;
  var PSIZE = Math.max(2, BSIZE - 4);
  var CHEIGHT = size * worldSize;
  var CWIDTH = size * worldSize;

  d3.selectAll("svg").remove();
  var chart = d3.select("#chart-wrapper").append("svg")
		.attr("class", "chart")
		.attr("width", CWIDTH)
		.attr("height", CHEIGHT);

  var xscale = d3.scale.linear()
		.domain([0, worldSize])
		.rangeRound([0, CWIDTH - BSIZE]);

  var yscale = d3.scale.linear()
		.domain([0, worldSize])
		.rangeRound([0, CHEIGHT - BSIZE]);

  var plants = data["plants"];
  chart.selectAll("rect")
      .data(plants)
      .enter().append("rect")
      .attr("x", function(d) { return xscale(d.x) + (BSIZE - PSIZE) / 2 })
      .attr("y", function(d) { return yscale(d.y) + (BSIZE - PSIZE) / 2 })
      .attr("width", function(d) { return PSIZE })
      .attr("height", function(d) { return PSIZE })
      .style("fill", function(d) { return "#284" });

  var creatures = data["creatures"];
  chart.selectAll("polygon")
		.data(creatures)
		.enter().append("polygon")
		.attr("points", function(d) {
			switch (d.b) {
        case 0:
          return (xscale(d.x) + BSIZE / 2) + "," + yscale(d.y) + " " +
              (xscale(d.x) + BSIZE) + "," + (yscale(d.y) + BSIZE) + " " +
              xscale(d.x) + "," + (yscale(d.y) + BSIZE);
        case 90:
          return xscale(d.x) + "," + yscale(d.y) + " " +
              (xscale(d.x) + BSIZE) + "," + (yscale(d.y) + BSIZE/2) + " " +
              xscale(d.x) + "," + (yscale(d.y) + BSIZE);
        case 180:
          return xscale(d.x) + "," + yscale(d.y) + " " +
              (xscale(d.x) + BSIZE) + "," + yscale(d.y) + " " +
							(xscale(d.x) + BSIZE / 2) + "," + (yscale(d.y) + BSIZE);
        case 270:
          return (xscale(d.x) + BSIZE) + "," + yscale(d.y) + " " +
              (xscale(d.x) + BSIZE) + "," + (yscale(d.y) + BSIZE) + " " +
              xscale(d.x) + "," + (yscale(d.y) + BSIZE/2);
      }})
		.style("fill", function(d) { return (d.adult ? "#CB4" : "#4BC") })
		.call(Crellinor.tooltip());

};

