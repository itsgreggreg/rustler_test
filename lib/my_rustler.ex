defmodule MyRustler do
  @moduledoc """
  This is the wrapping module for the myrustler crate in the native directory
  You must define the interface functions and 
  the use macro is where the magic happens.
  """

  # Where the magic happens
  use Rustler, otp_app: :greg, crate: :myrustler

  # Interface functions for calling and
  # error handling if the nif isn't loaded
  def add(_left, _right) do
    :erlang.nif_error(:nif_not_loaded)
  end

  # call this with true MyRustler.my_panic(true) to cause a rust panic
  def my_panic(_arg) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
