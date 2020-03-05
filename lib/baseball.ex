defmodule Baseball do
  @moduledoc """
  Example of a NIF that fakes a ML pattern
  """
  use Rustler, otp_app: :baseball, crate: "baseball"

  @doc """
  Sends in initial "model data"
  """
  @spec prepare(list(integer())) :: {:ok, reference()}
  def prepare(_nums), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Updates the state of the game
  """
  @spec update(reference(), :home_score | :away_score) :: :ok
  def update(_ref, _incident), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Gets the current score of the game
  """
  @spec get_scores(reference()) :: {:ok, integer(), integer()}
  def get_scores(_ref), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Takes in prepared data returned from prepare/1, with a multiplier
  and executes the predict function
  """
  def predict(_ref, _multiplier), do: :erlang.nif_error(:nif_not_loaded)
end
