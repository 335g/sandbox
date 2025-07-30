import marimo

__generated_with = "0.14.5"
app = marimo.App(width="medium")


@app.cell
def _():
    import marimo as mo
    return (mo,)


@app.cell
def _(mo):
    dates = mo.ui.date_range(start="2010-06-07", stop="2010-07-01")
    dates
    return (dates,)


@app.cell
def _(dates):
    from great_tables import GT
    from great_tables.data import sp500
    from datetime import datetime

    start_date = datetime.strftime(dates.value[0], format="%Y-%m-%d")
    end_date = datetime.strftime(dates.value[1], format="%Y-%m-%d")
    sp500_mini = sp500[(sp500["date"] >= start_date) & (sp500["date"] <= end_date)]

    (
        GT(sp500_mini)
            .tab_header(title="S&P 500", subtitle=f"{start_date} to {end_date}")
            .fmt_currency(columns=["open", "high", "low", "close"])
            .fmt_date(columns="date", date_style="wd_m_day_year")
            .fmt_number(columns="volume", compact=True)
            .cols_hide(columns="adj_close")
    )
    return


@app.cell
def _():
    from great_tables import nanoplot_options
    import polars as pl

    races = ["Blood Elf", "Troll", "Tauren", "Orc", "Undead"]
    wow_df = (
    
    )
    return


@app.cell
def _():
    return


if __name__ == "__main__":
    app.run()
