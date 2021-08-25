defmodule Ockam.Examples.TCP do
  @moduledoc """
  Example usage of tcp transport
  """
  alias Ockam.Transport.TCP
  alias Ockam.Transport.TCPAddress

  def server() do
    ## Start a transport with listener on port 4000
    TCP.start(listen: [port: 4000])
    __MODULE__.Printer.create(address: "printer")
  end

  def client() do
    ## Start a transport without a listener
    TCP.start()

    server_address = TCPAddress.new("localhost", 4000)

    Ockam.Router.route(%{
      onward_route: [server_address, "printer"],
      payload: "Hello!",
      return_route: []
    })
  end
end

defmodule Ockam.Examples.TCP.Printer do
  @moduledoc """
  An ockam worker to log all messages
  """
  use Ockam.Worker

  require Logger

  @impl true
  def handle_message(message, state) do
    Logger.info("Printer received: #{inspect(message)}")
    {:ok, state}
  end
end
