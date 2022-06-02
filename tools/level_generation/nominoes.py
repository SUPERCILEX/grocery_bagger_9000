import numpy as np


class Nomino:
    def __init__(self, name, matrix, num_rotations, origins):
        self.name = name
        self.matrix = np.array(matrix)
        self.rows, self.columns = self.matrix.shape
        self.num_rotations = num_rotations
        self.origins = origins

    def get_rotations(self):
        rotations = []
        for i in range(self.num_rotations):
            rotations.append(np.rot90(self.matrix, i))
        return rotations

    # Returns a list the form [np.array[[][]], int]
    def get_orientations(self):
        return zip(self.get_rotations(), self.origins)

    # origin is the x index of the topmost, leftmost block
    def get_origins(self):
        return self.origins

    def get_pieces(self):
        orientations = self.get_orientations()
        pieces = []
        for i in orientations:
            pieces.append(Piece(self.name, i))
        return pieces


class Domino(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "Domino",
                        [[x],
                         [x]],
                        2,
                        [0, 0])


class TrominoL(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TrominoL",
                        [[0, x],
                         [x, x]],
                        4,
                        [1, 0, 0, 0])


class TrominoStraight(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TrominoStraight",
                        [[x],
                         [x],
                         [x]],
                        2,
                        [0, 0])


class TetrominoT(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoT",
                        [[x, 0],
                         [x, x],
                         [x, 0]],
                        4,
                        [0, 1, 1, 0])


class TetrominoSquare(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoSquare",
                        [[x, x],
                         [x, x]],
                        1,
                        [0])


class TetrominoSkew(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoSkew",
                        [[x, x, 0],
                         [0, x, x]],
                        2,
                        [0, 1])


class TetrominoSkewMirror(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoSkew, Mirrored",
                        [[0, x, x],
                         [x, x, 0]],
                        2,
                        [1, 0])


class TetrominoL(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoL",
                        [[x, 0],
                         [x, 0],
                         [x, x]],
                        4,
                        [0, 2, 0, 0])


class TetrominoLMirror(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoL, Mirrored",
                        [[0, x],
                         [0, x],
                         [x, x]],
                        4,
                        [1, 0, 0, 0])


class TetrominoStraight(Nomino):
    def __init__(self, x=1):
        Nomino.__init__(self, "TetrominoStraight",
                        [[x],
                         [x],
                         [x],
                         [x]],
                        2,
                        [0, 0])


def GetNominoes(x):
    "this returns a list of Nominoes with the passed-in values"
    return [  # Domino(x),
        TrominoL(x),
        TrominoStraight(x),
        TetrominoT(x),
        TetrominoSquare(x),
        TetrominoSkew(x),
        TetrominoSkewMirror(x),
        TetrominoL(x),
        TetrominoLMirror(x),
        TetrominoStraight(x)]


class Piece():
    def __init__(self, name, orientation):
        self.name = name
        self.matrix = orientation[0].copy()
        self.rows, self.columns = self.matrix.shape
        self.offset = orientation[1]

    def __eq__(self, other):
        if not isinstance(other, Piece):
            return False
        return self.name == other.name

    ##    def __repr__(self):
    ##        return("piece!(Nomino::" + self.name +", Matrix: \n" + str(self.matrix) + ")\n")

    def __repr__(self):
        return self.name

    def print(self):
        print("\n\nName: " + self.name + "\nOffset: " + str(self.offset) + "\nMatrix:")
        print(self.matrix)

    def to_matrix(self, rows, columns, location=(0, 0)):
        start_row = location[0]
        start_column = location[1] - self.offset
        if start_row + self.rows > rows or start_column + self.columns > columns:
            return np.ones((rows, columns))
        new_matrix = np.zeros((rows, columns))

        for row in range(self.rows):
            for column in range(self.columns):
                new_matrix[start_row + row, start_column + column] = self.matrix[row, column]
        return new_matrix

    def value(self):
        return self.matrix[0, self.offset]

    def fits_in_matrix(self, bag_matrix, location):
        shape = np.shape(bag_matrix)
        m_rows = shape[0]
        m_columns = shape[1]
        loc_row = location[0]
        loc_column = location[1]

        if loc_column < self.offset:
            return False

        right_index = loc_column - self.offset + self.columns
        if right_index > m_columns:
            return False

        bottom_index = loc_row + self.rows
        if bottom_index > m_rows:
            return False

        piece_matrix = self.to_matrix(m_rows, m_columns, location)

        overlap = np.logical_and(piece_matrix > 0, bag_matrix > 0)

        return not np.any(overlap)


class LevelPiece():
    def __init__(self, name, color):
        self.name = name
        self.color = color

    # desired format:
    # piece!(mirrored Nomino::TetrominoL, NominoColor::Green)
    # piece!(Nomino::TetrominoL, NominoColor::Green)
    def __repr__(self):
        return ("piece!(Nomino::" + self.name + ", NominoColor::" + self.color + ")")
##def main():
##    nominos = GetNominoes(1)
##    for x in nominos:
##        for y in x.get_orientations():
##            print(Piece(x.name, y).to_matrix(5,5,[1,1 - y[1]]))
##    
##main()
