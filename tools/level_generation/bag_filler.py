from nominoes import *
import numpy as np

class Bag():
    def __init__(self, rows, columns):
        self.matrix = np.zeros((rows, columns))
        self.rows = rows
        self.columns = columns
        self.pieces = []
        self.last_placed = [0,-1]

    @classmethod
    def from_matrix(cls, matrix, pieces, location):
        rows, columns = np.shape(matrix)
        bag = cls(rows, columns)
        bag.add_matrix(matrix)
        bag.pieces += pieces.copy()
        bag.last_placed = location
        return bag

    def __eq__(self, other):
        if(isinstance(other, Bag)):
            return self.pieces == other.pieces
        return False

    def add_matrix(self, matrix):
        self.matrix = self.matrix + matrix

    def update_matrix(self, matrix):
        m_rows, m_columns = np.shape(matrix)
        if self.rows == m_rows and self.columns == m_columns:
            self.matrix = matrix.copy()

# precondition: every hole before this one is filled
    def fill_next_hole(self, depth=1, nominoes=None, loc=None):
        if loc == None:
            loc = self.last_placed.copy()
            loc[1] = loc[1] + 1
        row = loc[0]
        column = loc[1]
        if column >= self.columns:
            column = 0
            row = row + 1
        if row >= self.rows:
            return [self]
        if nominoes == None:
            nominoes = GetNominoes(depth)
        if self.matrix[row, column] != 0:
            return self.fill_next_hole(depth, nominoes, (row, column + 1))

        bags = []
        for n in nominoes:
            pieces = n.get_pieces()
            for p in pieces:
                placed = self.new_bag_with_piece_placed(p, [row, column])
                if placed[0]:
                    bags.append(placed[1])

        filled_bags = []
        for b in bags:
            filled_bags += b.fill_next_hole(depth + 1)
        return filled_bags

    def new_bag_with_piece_placed(self, piece, location):
        if not piece.fits_in_matrix(self.matrix, location):
            return [False]
        new_matrix = piece.to_matrix(self.rows, self.columns, location) + self.matrix
        new_pieces = self.pieces.copy() + [piece]
        return [True, Bag.from_matrix(new_matrix, new_pieces, location)]

##def main():
##    bag_a = Bag(4,4)
##    shape = Domino(1)
##    pieces = shape.get_pieces()
##    print(bag_a.try_place_piece(pieces[0]))
##    print(bag_a.try_place_piece(pieces[1]))
##    print(bag_a.try_place_piece(pieces[1], [0,3]))
##    bag_b = Bag(4,4)
##    bag_b.add_matrix(np.ones([4,4]))
##    print(bag_b.matrix + pieces[0].to_matrix(4,4))
##    print(bag_b.try_place_piece(pieces[0]))
##    shape = TetrominoT(1)
##    print("\n")
##    t_pieces = shape.get_pieces()
##    for piece in t_pieces:
##        print(bag_a.try_place_piece(piece, [0,3]))
##        print(bag_a.try_place_piece(piece, [0,0]))
##
##main()

def GetUniqueBagFillings(rows, columns):
    bag = Bag(rows, columns)
    list_o_bags = []
    bag_list = bag.fill_next_hole()
    return dedupe_bags(bag_list)

def dedupe_bags(bag_list):
    deduped_bags = []
    for bag in bag_list:
        dupe = False
        for other in deduped_bags:
            dupe = dupe or (bag == other)
        if dupe:
            continue
        deduped_bags.append(bag)
    return deduped_bags

##
##def main():
##    bag_list = GetUniqueBagFillings(4,3)
####    print("\n\n")
####    print(bag_list)
####    for bag in bag_list:
####        print(bag.matrix)
####        print(bag.pieces)
####    for bag in bag_list:
####        print(bag.matrix)
####        print(bag.pieces)
####    bags = dedupe_bags(bag_list)
##    for bag in bag_list:
####        print(bag.pieces)
####        print(bag.matrix)
####        print("")
####    print(len(bag_list))
##
##main()
