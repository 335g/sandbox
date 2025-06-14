import marimo

__generated_with = "0.13.15"
app = marimo.App(width="columns")


@app.cell(column=0)
def _():
    import marimo as mo
    import uvicorn
    import fastapi
    import requests as rq
    import threading
    import time
    import asyncio
    from mohtml import div, p, head, body, tailwind_css, html
    return asyncio, div, mo, p, rq, tailwind_css, threading, time, uvicorn


@app.cell
def _(mo, tailwind_css):
    PORT = 12345
    tailwind_css()

    get_server_thread, set_server_thread = mo.state(None)
    get_current_server, set_current_server = mo.state(None)
    return (
        PORT,
        get_current_server,
        get_server_thread,
        set_current_server,
        set_server_thread,
    )


@app.cell
def _(uvicorn):
    class StoppableServer:
        def __init__(self, app, host="0.0.0.0", port=8000):
            self.config = uvicorn.Config(app, host=host, port=port)
            self.server = uvicorn.Server(self.config)

        async def serve(self):
            await self.server.serve()

        def stop(self):
            if self.server:
                self.server.should_exit = True
                if hasattr(self.server, "servers"):
                    for server in self.server.servers:
                        server.close()
    return (StoppableServer,)


@app.cell
def _(
    PORT,
    StoppableServer,
    app,
    asyncio,
    get_current_server,
    get_server_thread,
    set_current_server,
    set_server_thread,
    threading,
    time,
):
    if get_server_thread() and get_server_thread().is_alive():
        print("Stopping current server...")
        if get_current_server():
            server = get_current_server()
            server.stop()
            set_current_server(server)

        thread = get_server_thread()
        thread.join(timeout=2)
        time.sleep(0.5)

    set_current_server(StoppableServer(app, host="0.0.0.0", port=PORT))


    def run_server():
        asyncio.run(get_current_server().serve())


    current_server_thread = threading.Thread(target=run_server, daemon=True)
    current_server_thread.start()
    set_server_thread(current_server_thread)
    return


@app.cell
def _(get_server_thread):
    get_server_thread()
    return


@app.cell(column=1)
def _(div, p):
    from fastapi.responses import HTMLResponse, JSONResponse
    from fastapi import FastAPI, Response

    app = FastAPI()


    @app.get("/", response_class=HTMLResponse)
    def read_root():
        return str(div(p("hello", klass="text-2xl text-center text-blue-500")))


    @app.get("/api", response_class=JSONResponse)
    def read_root():
        return {"alive": "yes!!"}
    return (app,)


@app.cell
def _(PORT, div, rq):
    div(rq.get(f"http://localhost:{PORT}/").text)
    return


@app.cell
def _(PORT, rq):
    rq.get(f"http://localhost:{PORT}/api").json()
    return


@app.cell
def _():
    return


if __name__ == "__main__":
    app.run()
