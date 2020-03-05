defmodule BaseballTest do
  use ExUnit.Case

  test "test" do
    nums = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]

    assert {:ok, ref} = Baseball.prepare(nums)
    assert is_reference(ref)

    assert :ok = Baseball.update(ref, :home_score)
    assert :ok = Baseball.update(ref, :away_score)
    assert :ok = Baseball.update(ref, :home_score)
    assert :ok = Baseball.update(ref, :home_score)

    assert {:ok, 3, 1} = Baseball.get_scores(ref)

    assert {:ok, 0.118} = Baseball.predict(ref, 2.0)
  end
end
