import marimo

__generated_with = "0.13.15"
app = marimo.App(width="medium")


@app.cell
def _():
    import marimo as mo
    return


@app.cell
def _():
    import polars as pl

    penguins = pl.read_csv("https://github.com/juba/pyobsplot/raw/main/doc/data/penguins.csv")

    return (penguins,)


@app.cell
def _(penguins):
    from pyobsplot import Plot

    Plot.plot({
        "grid": True,
        "color": {"legend": True},
        "marks": [
            Plot.dot(penguins, {
                "x": "flipper_length_mm",
                "y": "body_mass_g",
                "fill": "species"
            }),
            Plot.density(penguins, {
                "x": "flipper_length_mm",
                "y": "body_mass_g",
                "stroke": "species"
            }),
        ]
    })
    return


@app.cell
def _():
    return


if __name__ == "__main__":
    app.run()
