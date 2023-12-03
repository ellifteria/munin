using Plots, CSV, DataFrames
theme(:wong)

data_files = [
	("./output/add.csv", "ADD (LSPACE algorithm)"),
	("./output/lin-add.csv", "ADD (linear algorithm)"),
	("./output/pal-add.csv", "PAL-ADD"),
	("./output/pal.csv", "PAL"),
]

for file in data_files

	path = file[1]
	title = file[2]

	add_data = DataFrame(
	CSV.File(
		path,
		header = false))
	p21 = plot(add_data[!, 1], add_data[!, 3])
	plot!(add_data[!, 1], add_data[!, 3], seriestype="scatter")
	plot!(legend=false)
	plot!(ylabel="Execution-time memory used (bits)")
	plot!(title=title)

	p22 = plot(add_data[!, 1], add_data[!, 3])
	plot!(add_data[!, 1], add_data[!, 3], seriestype="scatter")
	plot!(legend=false)
	plot!(title=title)
	plot!(xscale=:log10, minorgrid=true)
	
	plot(p21, p22, layout=(1,2), legend=false, plotdensity=10000, dpi=600)
	plot!(xlabel="Input length (bits)")
	png("writeup/analysis/" * replace(title, " " => "-"))
end
