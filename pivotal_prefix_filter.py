import ukkonen
from functools import cache
import time
import multiprocessing


def ngramify(doc: str, n: int, lt: int = -1) -> list[tuple[str, int]]:
    ngrams = []
    t = lt if lt >= 0 else threshold
    # padding to make sure we have enough ngrams
    if len(doc) - n + 1 - (n * t) <= 0:
        doc += '$' * (abs(len(doc) - n + 1 - (n * t)) + 1)
    ngram_count = len(doc) - n + 1
    for i in range(ngram_count):
        ngram = doc[i:i + n]
        ngrams.append((ngram , i))
    return ngrams


# with open('./workloads/mini/input') as f:
# with open('./workloads/local1/input') as f:
# with open('./workloads/cities-2') as f:
with open('./workloads/cities-mini') as f:
    data = [l.strip() for l in f.readlines()]
    
srch_id = data.index('[SEARCH]')

db = [(l, id + 1, len(l)) for id, l in enumerate(data) if id < srch_id]
qdb = [l for id, l in enumerate(data) if id > srch_id]
threshold = max([int(q.split(',')[1]) for  q in qdb])

# db = [('Ozero Uvay', 1, 10)]
# qdb = ['Ozero Umay,1']
# threshold = 6

occurence_map = dict()

# threshold = 3

# db = ['imyouteca', 'ubuntucom', 'utubbecou', 'youtbecom', 'yoytubeca']
# query = 'yotubecom'

pref_ivx = [dict() for _ in range(threshold + 1)]
piv_ivx = [dict() for _ in range(threshold + 1)]
pref_len_map = [dict() for _ in range(threshold + 1)]
piv_len_map = [dict() for _ in range(threshold + 1)]

n = 2
ngram_prefix_set = []

def global_order_sort(gram):
    return occurence_map.get(gram[0], [1, len(occurence_map.keys()) + 1])


def prefix_selection(ngrams, threshold):
    prefix_len = threshold * n + 1
    ngrams.sort(key=global_order_sort)
    return ngrams[:prefix_len]


# r3 = utubbecou
# s = yotubecom
def pivots_selection(pfset: list[tuple[str, int]], threshold: int, pfset_weights: list[int]):

    # min_weight = min(pfset_weights)
    # min_weight_idx = pfset_weights.index(min_weight)
    
    # mem_weights = []
    # mem_pivots = []
    # for _ in range(len(pfset)):
    #     mem_weights.append([9999] * (threshold + 1))
    #     mem_pivots.append([[]] * (threshold + 1))
    #     mem_weights[-1][0] = min_weight
    #     mem_pivots[-1][0] = [pfset[min_weight_idx]]
    
    # cache = dict()
    
    @cache
    def optimal_selection(i, j):
        if j == 0 and i >= j:
            mn = min(pfset_weights[:i + 1])
            mnidx = pfset_weights[:i + 1].index(mn)
            return pfset_weights[mnidx], [pfset[mnidx]]
        if i < j:
            return 9999999999999, []
        
        minimal_k = len(pfset) + 1
        min_w = 9999999999999
        min_pivots = []
        for k in range(j, i + 1):
            _, kgram_pos = pfset[k]
            l = k - 1
            _, lgram_pos = pfset[k - 1]
            if abs(kgram_pos - lgram_pos) < n:
                l = k - 2
            weight, candidate_pivot = optimal_selection(l, j - 1)
            
            if weight + pfset_weights[k] < min_w:
                minimal_k = k
                min_w = weight + pfset_weights[k]
                min_pivots = candidate_pivot
        
        if minimal_k > len(pfset):
            return 9999999999999, []
        return min_w, min_pivots + [pfset[minimal_k]]
    
    
    weight, pivots = optimal_selection(len(pfset) - 1, threshold)

    pivots.sort(key=global_order_sort)
    return pivots

# sorting the strings messes up their real id
db.sort(key=lambda x: len(x[0]))

# for idx, (w, wid, wlen) in enumerate(db):
#     if w == 'Bogomolov':
#         print('Bogomolov ID is: ', idx)
#         break

smallest_len = len(db[0][0])

# if smallest_len - n + 1 - (n * threshold) <= 0:
#     # TODO Pad such small string ngrams
#     print('Strings are too small to support qgram pruning')
#     exit(0)

ngram_db = [ngramify(rec, n) for rec, _, __ in db]

for i in range(0, len(ngram_db), 2):
    ngram_set = ngram_db[i]
    for ngram, ng_pos in ngram_set:
        if ngram not in occurence_map:
            occurence_map[ngram] = [0, len(occurence_map.keys()) + 1]
        occurence_map[ngram][0] += 1
        # occurence_map[ngram] += 1

# print(pivots_selection([('ut', 0), ('ub', 2), ('bb', 3), ('tu', 1), ('ou', 7)], threshold))

for str_id, ngram_set in enumerate(ngram_db):
    ngram_prefix_set.append(prefix_selection(ngram_set, threshold))        
    
    # print(ngram_prefix_set[-1])

# for qword in qdb[:-2]:
    # query, _ = qword.split(',')
    


# print('Query: ', pref_qngrams)

# for idx, ngrams in enumerate(ngram_prefix_set):
#     if len(set(ngram for ngram, _ in ngrams) & set(ngram for ngram, _ in pref_qngrams)) == 0:
#         print('Filtered', db[idx], ukkonen.distance(query, db[idx], threshold + 1))

# ngram_prefix_set = [ngrams for ngrams in ngram_prefix_set if len(set(ngrams) & set(pref_qngrams)) > 0]
print('Building indexes')
start = time.time()
pivot_ngrams = []
for str_id, ngram_set in enumerate(ngram_prefix_set):

    pfset = ngram_set.copy()
    pfset.sort(key=lambda x: x[1])
    pfset_weights = [occurence_map.get(ngram, [1])[0] for ngram, _ in pfset]
    pivots = pivots_selection(pfset, threshold, pfset_weights)
    if len(pivots) == 0:
        print(f'Unable to get pivots for: {str_id} -- {db[str_id]}')
        continue
    if db[str_id][0] == 'Ozero Uvay':
        print(ngram_set)
        print(pivots)
    
    g_0 = ngram_db[str_id][0]
    
    if g_0[0] not in piv_ivx[0]:
        piv_ivx[0][g_0[0]] = []
    piv_ivx[0][g_0[0]].append((str_id, g_0[1]))

    if g_0[0] not in pref_ivx[0]:
        pref_ivx[0][g_0[0]] = []
    pref_ivx[0][g_0[0]].append((str_id, g_0[1]))
        # piv_len_map[piv] = dict()
    
    # threshold here is max threshold
    piv_ivx_gram_set = [g_0] + pivots[:threshold]
    if g_0 == pivots[0]:
        piv_ivx_gram_set = pivots[:threshold + 1]
    
    g_i_1_pref_idx = 0
    g_i_pref_idx = 0 # ngram_set.index(piv_ivx_gram_set[1])
    
    # t is current threshold
    for t in range(1, threshold + 1):
        g_i = piv_ivx_gram_set[t]
        g_i_1 = piv_ivx_gram_set[t - 1]
        # if db[str_id][0] == 'Ozero Uvay':
        #     print(f'PIV[{t}][{g_i[0]}] = (, {g_i[1]})')
        
        for idx in range(g_i_pref_idx, len(ngram_set)):
            if g_i == ngram_set[idx]:
                g_i_pref_idx = idx
                break
        
        if g_i[0] not in piv_ivx[t]:
            piv_ivx[t][g_i[0]] = []
            piv_len_map[t][g_i[0]] = dict()
        piv_ivx[t][g_i[0]].append((str_id, g_i[1]))
        
        str_id_len = db[str_id][2]
        if str_id_len not in piv_len_map[t][g_i[0]]:
            piv_len_map[t][g_i[0]][str_id_len] = len(piv_ivx[t][g_i[0]]) - 1
        
        # prefix grams to insert
        pref_ins = ngram_set[g_i_1_pref_idx + 1:g_i_pref_idx + 1]
        
        for pref_gram, gram_pos in pref_ins:
            if pref_gram not in pref_ivx[t]:
                pref_ivx[t][pref_gram] = []
                pref_len_map[t][pref_gram] = dict()
            
            # if db[str_id][0] == 'Ozero Uvay':
            #     print(f'PREF[{t}][{pref_gram}] = (, {gram_pos})')
            
            pref_ivx[t][pref_gram].append((str_id, gram_pos))
            if str_id_len not in pref_len_map[t][pref_gram]:
                pref_len_map[t][pref_gram][str_id_len] = len(pref_ivx[t][pref_gram]) - 1
            
        g_i_1_pref_idx = g_i_pref_idx
    
    
    # appending to pivot index
    # for piv, piv_pos in pivots:
    #     if len(db[str_id]) not in piv_len_map[piv]:
    #         piv_len_map[piv][len(db[str_id])] = len(pref_ivx[piv]) - 1
            
    
    # # Appending to prefix index
    # for pref_gram, gram_pos in ngram_prefix_set[-1]:
    #     if pref_gram not in pref_ivx:
    #         pref_ivx[pref_gram] = []
    #         pref_len_map[pref_gram] = dict()
            
    #     pref_ivx[pref_gram].append((str_id, gram_pos))
        
    #     # building the length filter hash map
    #     if len(db[str_id]) not in pref_len_map[pref_gram]:
    #         pref_len_map[pref_gram][len(db[str_id])] = len(pref_ivx[pref_gram]) - 1
    pivot_ngrams.append(pivots)
    # print(pivot_ngrams[-1])

end = time.time()

print(f'Indexes built in {(end - start) * 1000}ms')

################ This is the pivotal prefix filter ####################
# for idx, piv_grams in enumerate(pivot_ngrams):
#     pref_r = ngram_prefix_set[idx]
#     pref_s = pref_qngrams
    
#     if occurence_map.get(pref_r[-1][0], 1) > occurence_map.get(pref_s[-1][0], 1):
#         if set(p[0] for p in query_pivots) & set(p[0] for p in pref_r) == set():
#             print('Piv prefix filtered out', db[idx], ukkonen.distance(db[idx], query, threshold + 1))
#     else:
#         if set(p[0] for p in piv_grams) & set(p[0] for p in pref_qngrams) == set():
#             print('Piv prefix filtered out', db[idx], ukkonen.distance(db[idx], query, threshold + 1))


# qdb = [f'{query},2']
lve_sum = 0
lve_sum_2 = 0

def query_return_results(qword):
    lve_sum = 0
    query, t_ = qword.split(',')
    threshold = int(t_)
    qngrams = ngramify(query, n, threshold)
    pref_qngrams = prefix_selection(qngrams, threshold)
    
    pfset = pref_qngrams.copy()
    pfset.sort(key=lambda x: x[1])
    pfset_weights = [0] * len(pfset)
    for t in range(threshold + 1):
        for idx, (qgram, _) in enumerate(pfset):
            pfset_weights[idx] += len(pref_ivx[t].get(qgram, []))
    
    
    query_pivots = pivots_selection(pfset, threshold, pfset_weights)
    # Searching the indexes

    candidate_set = set()

    # first search by query prefixes
    for qpref_gram, qgram_pos in pref_qngrams:
        for t in range(threshold + 1):
            if qpref_gram not in piv_ivx[t]:
                continue
            
            listings = piv_ivx[t][qpref_gram]
            last_gram_idx = (n * t + 1) - 1
            # start = piv_len_map[t].get(qpref_gram, dict()).get(len(query) - threshold, 0)
            # end = piv_len_map[t].get(qpref_gram, dict()).get(len(query) + threshold + 1, len(piv_len_map[t].get(qpref_gram, dict()))) - 1
            
            for rec_id, cpos in listings:
                if global_order_sort(ngram_prefix_set[rec_id][last_gram_idx][0]) <= global_order_sort(pref_qngrams[last_gram_idx][0]) \
                    and abs(cpos - qgram_pos) <= threshold:
                        candidate_set.add(rec_id)

    # then search by query pivots
    for qpiv_gram, qpiv_pos in query_pivots:
        for t in range(threshold + 1):
            if qpiv_gram not in pref_ivx[t]:
                continue
            
            listings = pref_ivx[t][qpiv_gram]
            last_gram_idx = (n * t + 1) - 1
            
            # start = pref_len_map[t].get(qpiv_gram, dict()).get(len(query) - threshold, 0)
            # end = pref_len_map[t].get(qpiv_gram, dict()).get(len(query) + threshold + 1, len(pref_len_map[t].get(qpiv_gram, dict()))) - 1
            # for i in range(start, end + 1):
            for rec_id, cpos in listings:
                if global_order_sort(ngram_prefix_set[rec_id][last_gram_idx][0]) > global_order_sort(pref_qngrams[last_gram_idx][0]) \
                    and abs(cpos - qpiv_pos) <= threshold:
                        candidate_set.add(rec_id)

    # print(f'Candidates for query {query} with threshold {threshold}:', len(candidate_set))
    real_candidates = []
    missing_sum = 0
    selected_candidates = []
    for candidate in candidate_set:
        word, wid, _ = db[candidate]
        candidate_distance = ukkonen.distance(word, query, threshold + 1)
        # print('Candidate distance:', word, query, candidate_distance)
        if candidate_distance <= threshold:
            lve_sum += wid
            selected_candidates.append(word)
            
    for idx, rec in enumerate(db):
        if ukkonen.distance(query, rec[0], threshold + 1) <= threshold:
            real_candidates.append(rec[0])
    
    diff = set(real_candidates) - set(selected_candidates)
    if len(diff) > 0:
        print(f'Diff in candidates for query {query} and {threshold} is: {len(diff)} {diff}')
        for word in diff:
            for idx, rec in enumerate(db):
                if rec[0] == word:
                    missing_sum += rec[1]
                    break
        
        
    return lve_sum, missing_sum



with multiprocessing.Pool(6) as p:
    results = p.map(query_return_results, qdb)
    lve_sum = sum(l[0] for l in results)
    missing = sum(l[1] for l in results)


# print(query_return_results('Ozero Umay,1'))
    # real_ones = []query_return_results('Bogomolovo,1')
# Diff in candidates for query Bogomolovo is: 1 {'Bogomolov'}



print('Result sum: ', lve_sum, 'missing', missing, 'together', missing + lve_sum)

