import unittest
import euklid_rs


class TestCase(unittest.TestCase):
    """Base TestCase"""

    def assert_almost_equal_vec(self, vec_1, vec_2, places: int = 8):
        """Check equality of two vectors"""
        if isinstance(vec_1, euklid_rs.vector.Vector3D):
            dimension = 3
        elif isinstance(vec_1, euklid_rs.vector.Vector2D):
            dimension = 2
        else:
            raise AssertionError(f"wrong type: {type(vec_1)}")
        try:
            for i in range(dimension):
                self.assertAlmostEqual(vec_1[i], vec_2[i], places)
        except AssertionError as exception:
            raise AssertionError(f"{vec_1} != {vec_2}") from exception
