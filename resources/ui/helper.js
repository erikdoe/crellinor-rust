
Crellinor = {}

Crellinor.tooltip = function(a) {
	
	var accessor = arguments.length ? a : undefined;
	
	function tooltip(selection) {
		selection
			.on("mouseover", function(d) {
				if(accessor) {
					d = accessor(d);
				}
			 	var div = d3.select("body").selectAll("div.tooltip");
				if (div.empty()) {
				 	div = d3.select("body").append("div").attr("class", "tooltip").style("opacity", 0);
				}
			  div.html("");
        if (d.fname) {
          div.append("h2").text(d.fname.split("/").slice(-1)[0]);
				  div.append("p").attr("class", "filename").text(d.fname);
        }
				for (var p in d) {
        	if (p === "program") {
            div.append("pre").text(d[p].replace(/(; )/g, "\n"));
          }
				  else if (d.hasOwnProperty(p)) {
						div.append("p").text(p + ": " + d[p]);
				  }
				}
				var ttx = d3.event.pageX;
				var tty = d3.event.pageY - $("div.tooltip").height() - 15;
				var hclip = (ttx + $("div.tooltip").width()) - ($(window).width() + $(window).scrollLeft())
				if (hclip > 0) {
					ttx -= hclip
				}
				div.style("left", Math.max(ttx - 20, $(window).scrollLeft() + 5) + "px")     
	  		   .style("top", Math.max(tty, $(window).scrollTop() + 5) + "px");
 				div.transition().duration(100).style("opacity", 0.95);
			})
			.on("mouseout", function(d) {       
				div = d3.select("body").select("div.tooltip")
				div.transition().duration(250).style("opacity", 0);
			});
	}
	
	return tooltip;
	
};

		
Array.prototype.shuffle = function() {
	var i = this.length, j, tempi, tempj;
	if (i === 0) 
		return false;
	while (--i) {
		j = Math.floor( Math.random() * ( i + 1 ) );
		tempi = this[i];
		tempj = this[j];
		this[i] = tempj;
		this[j] = tempi;
	}
	return this;
}
