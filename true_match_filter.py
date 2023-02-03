import ukkonen
import bisect
from math import ceil

class NGramFilter:
    TRANSLATE_MAP = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52,  # 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53,  # 48
        54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 0,  # 64
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,  # 80
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0,  # 96
        0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,  # 112
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0,  # 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 160
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 176
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 192
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 208
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 224
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 240
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 256
    ]
    SIGMA = 63

    Q = 1
    
    def __init__(self, doc: str) -> None:
        self.ranking_profile = [0] * NGramFilter.SIGMA
        sdist = len(doc) - NGramFilter.Q + 1
        self.ranking_profile[NGramFilter.TRANSLATE_MAP[ord(doc[0])]] += 1
        for s_i in doc[1:sdist]:
            r = NGramFilter.TRANSLATE_MAP[ord(s_i)]
            self.ranking_profile[r] += 1
            
    @classmethod
    def dist(cls, f1, f2) -> int:
        return sum(abs(a - b) for a, b in zip(f1.ranking_profile, f2.ranking_profile))
    


TRANSLATE_MAP = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 16
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52,  # 32
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53,  # 48
    54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 0,  # 64
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,  # 80
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0,  # 96
    0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,  # 112
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0,  # 128
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 144
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 160
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 176
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 192
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 208
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 224
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 240
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  # 256
]
SIGMA = 63


class TrueMatchFilter:
    def __init__(self, n: int, doc: str, _id: int):
        self.id = _id
        self.n = n
        self.chunks = []
        self.str_len = len(doc)
        self.lbstr = ceil(len(doc) / n)
        self.chunks = nchunkify(doc, n)
        
            
    def matches(self, ngram_list: list[tuple[str, int]], threshold: int) -> bool:
        lb = self.lbstr - threshold
        mismatch = 0
        M: list[tuple[str, str, int]] = []
        last_chunk = None
        lo = 0        
        for idx, (chunk, chunk_pos) in enumerate(self.chunks):
            if last_chunk != chunk:
                last_chunk = chunk
                lo = idx
            match_idx = bisect.bisect_left(ngram_list, (chunk,), lo=lo)
            if match_idx >= len(ngram_list):
                match_ngram = None
            elif chunk == ngram_list[match_idx][0]:
                match_ngram, ngram_pos = ngram_list[match_idx]
            else:
                match_ngram = None
            
            if match_ngram is None:
                mismatch += 1
                if mismatch > len(self.chunks) - lb:
                    return False
            else:
                while match_ngram == chunk:
                    if abs(chunk_pos - ngram_pos) <= threshold:
                        M.append((chunk, ngram_pos, chunk_pos))
                    match_idx += 1
                    if match_idx >= len(ngram_list):
                        break
                    match_ngram, ngram_pos = ngram_list[match_idx]
        
        f = len(M) >= lb
        return f if not f else self.true_match(M, lb)
        return self.true_match(M, lb)
    
    def true_match(self, M: list[tuple[str, str, int]], lb: int) -> bool:
        M.sort(key=lambda x: (x[1], x[2]))
        M.insert(0, None)
        opt = [0] * len(M)
        
        # first in tuple is chunk, second is ngram
        def compatible(ei: tuple[str, int, int], ej: tuple[str, int, int]) -> bool:
            if ej is None:
                return True
            if ei[2] != ej[2] and ei[1] >= (ej[1] + self.n):
                return True
            # print(ei, ' and ', ej, ' = incompatible! = ', f'{ei[2]} != {ej[2]}', '\t' f'{ei[1]} >= {ej[1] + self.n}')
            return False
            
        
        for k in range(1, len(M)):
            mx = -9999
            mn = min(k, len(M) - lb + 1)
            for i in range(1, mn + 1):
                if compatible(M[k], M[k - i]) and opt[k - i] > mx:
                    mx = opt[k - i] + 1
            opt[k] = mx
        return max(opt[lb:]) >= lb


def nchunkify(doc: str, n: int) -> list[tuple[str, int]]:
    global TRANSLATE_MAP
    global SIGMA
    # padding to make sure we have enough chunks
    if len(doc) % n != 0:
        doc += '$' * (n - (len(doc) % n))
    chunks = []
    total_chunks: int = int(ceil(len(doc) / n))
    for i in range(0, total_chunks):
        nchunk = doc[i * n: i * n + n]
        # nchunk_num = (TRANSLATE_MAP[ord(nchunk[0])] * SIGMA) + TRANSLATE_MAP[ord(nchunk[1])]
        
        chunks.append((nchunk, i * n))
    chunks.sort(key=lambda x:(x[0], -x[1]))
    return chunks

    
def ngramify(doc: str, n: int) -> list[tuple[str, int]]:
    global TRANSLATE_MAP
    global SIGMA
    ngrams = []
    # padding to make sure we have enough ngrams
    doc += '$' * (n - 1)
    ngram_count = len(doc) - n + 1
    for i in range(ngram_count):
        ngram = doc[i:i + n]
        # ngram_num = (TRANSLATE_MAP[ord(ngram[0])] * SIGMA) + TRANSLATE_MAP[ord(ngram[1])]
        ngrams.append((ngram , i))
        # bisect.insort(ngrams, (doc[i:i + n], i))
    ngrams.sort(key=lambda x:(x[0], -x[1]))
    return ngrams


class IndexChunk:
    def __init__(self, threshold: int):
        self.t = threshold
                                     # str_len, chunk offset, record id
        self.listings: dict[str, list[int, int, int]] = dict()
        
    def append(self, chunk_filter: TrueMatchFilter):
        for i in range(0, self.t + 1):
            if chunk_filter.chunks[i][0] not in self.listings:
                self.listings[chunk_filter.chunks[i][0]] = []
            self.listings[chunk_filter.chunks[i][0]].append((chunk_filter.str_len, chunk_filter.chunks[i][1], chunk_filter.id))
            
    def query(self, query, threshold: int) -> list[int]:
        sigs = ngramify(query, 5)
        prefix_sig_count = len(query) - (ceil((len(query) - threshold) / 5) - threshold) + 1
        prefix_sigs = sigs[:prefix_sig_count]
        candidates = []
        for sig_gram, sig_pos in prefix_sigs:
            for candidate in self.listings.get(sig_gram, []):
                cnd_str_len, cnd_pos, cnd_id = candidate
                if cnd_id not in candidates and abs(cnd_str_len - len(query)) <= threshold and abs(cnd_pos - sig_pos) <= threshold:
                    candidates.append(cnd_id)
        return candidates
            
        
        
    


# with open('./workloads/mini/input') as f:
with open('./workloads/local1/input') as f:
    data = [l.strip() for l in f.readlines()]
    
srch_id = data.index('[SEARCH]')

db = [l for id, l in enumerate(data) if id < srch_id]
qdb = [l for id, l in enumerate(data) if id > srch_id]

filters: list[NGramFilter] = [NGramFilter(doc) for doc in db]
n = 5
true_match_filters: list[TrueMatchFilter] = [TrueMatchFilter(n, doc, idx + 1) for idx, doc in enumerate(db)]

max_threshold = max([int(l.split(',')[1]) for l in qdb])
indexes: list[IndexChunk] = []

print('---- Building indexes -----')
for i in range(max_threshold + 1):
    indexes.append(IndexChunk(i))
    for tmf in true_match_filters:
        indexes[-1].append(tmf)
print('----- Index built -------')

def flatten(l):
    return [item for sublist in l for item in sublist]

# qs, threshold = qdb[0].split(',')
ngrams = ngramify('abcdcdab', 2)
# print(ngrams)
# ft = TrueMatchFilter(2, 'TAGTATTCTCTTACCTTCTGGATATTAGGAACAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT')
# mt = ft.matches(ngramify('TAGTATTCTCTTACCTTCTGGATATTAGGAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT', 2), 12)
# print(mt)
# # print(true_match_filters[0].chunks)

# ft = TrueMatchFilter(2, 'abcdcdab')
# mt = ft.matches(ngramify('bccdabcd', 2), 2)
# print(mt)


# ft = TrueMatchFilter(2, 'kafe')
# mt = ft.matches(ngramify('dale', 2), 2)
# print(mt)


# ft = TrueMatchFilter(2, 'karel')
# mt = ft.matches(ngramify('kryl', 2), 2)
# print(mt)

# Query - CTCTGTTGCCCAGGCTGGAGTGCACTGGCGTGAGTCTCGGCTCACTGCAACCTCTGCTTCCCAGGTTTAAGCGATTCTCCTGCTTCAGCCTCCCAAGTAGC 
# Record - GCTCTGTCGCCCAGGCTGGAGTGCAGTGGCATGATCTCGGCTCACTGCAACCTCCACCTCCCAGGTTCAAGTGATTCTCCTGCCTCAGCCTCCCGAGTAGC 
# ft = TrueMatchFilter(2, 'GCTCTGTCGCCCAGGCTGGAGTGCAGTGGCATGATCTCGGCTCACTGCAACCTCCACCTCCCAGGTTCAAGTGATTCTCCTGCCTCAGCCTCCCGAGTAGC')
# mt = ft.matches(ngramify('CTCTGTTGCCCAGGCTGGAGTGCACTGGCGTGAGTCTCGGCTCACTGCAACCTCTGCTTCCCAGGTTTAAGCGATTCTCCTGCTTCAGCCTCCCAAGTAGC', 2), 12)



total_sum = 0
for query in qdb:
    qs, threshold = query.split(',')
    threshold = int(threshold)
    candidates = [index.query(qs, threshold) for index in indexes if index.t <= threshold]
    candidates = set(flatten(candidates))
    
    print('Candidates: ', len(candidates))
    filtered = [(db[candidate - 1], candidate - 1) for candidate in candidates]
    for word, idx in filtered:
        distance = ukkonen.distance(qs, word, threshold + 1)
        if distance <= threshold:
            # print('Matching: ', qs, word, threshold)
            total_sum += idx + 1
        # print(qs, '\t', word, ' = ', distance)
        
print('Total sum Filter: ', total_sum)
