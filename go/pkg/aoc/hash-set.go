package aoc

type HashSet[K comparable] map[K]struct{}

func NewHashSetFromSlice[K comparable](vs []K) HashSet[K] {
	set := HashSet[K]{}

	for _, v := range vs {
		set[v] = struct{}{}
	}

	return set
}

func (self HashSet[K]) Contains(v K) bool {
	_, ok := self[v]

	return ok
}

func (self HashSet[K]) Keys() []K {
	keys := []K{}

	for v := range self {
		keys = append(keys, v)
	}

	return keys
}

func (self HashSet[K]) Union(other HashSet[K]) HashSet[K] {
	union := HashSet[K]{}

	for v := range self {
		union[v] = struct{}{}
	}

	for v := range other {
		union[v] = struct{}{}
	}

	return union
}

func (self HashSet[K]) Intersection(other HashSet[K]) HashSet[K] {
	intersection := HashSet[K]{}

	for v := range self {
		if other.Contains(v) {
			intersection[v] = struct{}{}
		}
	}

	return intersection
}
