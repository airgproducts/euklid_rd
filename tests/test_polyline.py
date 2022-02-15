#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""

import math
import unittest
from euklid_rs.polyline import PolyLine2D
from euklid_rs.vector import Vector2D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        # Vector2D
        self.line = PolyLine2D.from_list([[0,0],[1,0],[2,1],[1,3],[0,0]])

    def test_get(self):
        self.assertEqual(self.line.get(0), Vector2D([0,0]))
        self.assertEqual(self.line.get(1), Vector2D([1,0]))
        self.assertEqual(self.line.get(1.5), Vector2D([1.5,0.5]))
        self.assertEqual(self.line.get(-1), Vector2D([-1,0]))



if __name__ == '__main__':
    unittest.main(exit=False)
