import ukkonen
from functools import cache
import time

def ngramify(doc: str, n: int) -> list[tuple[str, int]]:
    ngrams = []
    # padding to make sure we have enough ngrams
    doc += '$' * (n - 1)
    ngram_count = len(doc) - n + 1
    for i in range(ngram_count):
        ngram = doc[i:i + n]
        ngrams.append((ngram , i))
    return ngrams


# with open('./workloads/mini/input') as f:
with open('./workloads/local1/input') as f:
    data = [l.strip() for l in f.readlines()]
    
srch_id = data.index('[SEARCH]')

db = [l for id, l in enumerate(data) if id < srch_id]
qdb = [l for id, l in enumerate(data) if id > srch_id]
threshold = max([int(q.split(',')[1]) for  q in qdb])
occurence_map = dict()

# threshold = 3

# db = ['imyouteca', 'ubuntucom', 'utubbecou', 'youtbecom', 'yoytubeca']
# query = 'yotubecom'

pref_ivx = [dict() for _ in range(threshold + 1)]
piv_ivx = [dict() for _ in range(threshold + 1)]
pref_len_map = dict()
piv_len_map = dict()

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
    
    # @cache
    # def optimal_selection(i, j):
    #     if j == 0 and i >= j:
    #         mn = min(pfset_weights[:i + 1])
    #         mnidx = pfset_weights[:i + 1].index(mn)
    #         return pfset_weights[mnidx], [pfset[mnidx]]
    #     if i < j:
    #         return 9999999999999, []
        
    #     minimal_k = len(pfset) + 1
    #     min_w = 9999999999999
    #     min_pivots = []
    #     # if (is_traced and j <= 5) or (i == 6 and j == 3) or (i == 2 and j == 1):
    #     #     print(f'Getting for i={i} and j={j}')
    #     for k in range(j, i + 1):
    #         # if (k == 25 and j == 16) or is_traced:
    #         #     print('Cus')
    #         _, kgram_pos = pfset[k]
    #         l = k - 1
    #         _, lgram_pos = pfset[k - 1]
    #         if abs(kgram_pos - lgram_pos) < n:
    #             l = k - 2
    #         weight, candidate_pivot = optimal_selection(l, j - 1)
    #         # if is_traced:
    #         #     print(f'Result for k={k}/ l={l} and j={j - 1} is {weight} {candidate_pivot}')
            
    #         if weight + pfset_weights[k] < min_w:
    #             minimal_k = k
    #             min_w = weight + pfset_weights[k]
    #             min_pivots = candidate_pivot
        
    #     if minimal_k > len(pfset):
    #         return 9999999999999, []
    #     return min_w, min_pivots + [pfset[minimal_k]]
    
    
    # weight, pivots = optimal_selection(len(pfset) - 1, threshold)
    pivots = []
    pivots.append(pfset[0])
    for ngram, ngram_pos in pfset[1:]:
        for pn in pivots:
            if abs(pn[1] - ngram_pos) < n:
                break
        else:
            pivots.append((ngram, ngram_pos))
            
        if len(pivots) == threshold + 1:
            break
    pivots.sort(key=global_order_sort)
    return pivots


db.sort(key=lambda x: len(x))

ngram_db = [ngramify(rec, n) for rec in db]

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
        
        for idx in range(g_i_pref_idx, len(ngram_set)):
            if g_i == ngram_set[idx]:
                g_i_pref_idx = idx
                break
        
        if g_i[0] not in piv_ivx[t]:
            piv_ivx[t][g_i[0]] = []
        piv_ivx[t][g_i[0]].append((str_id, g_i[1]))
        
        # prefix grams to insert
        pref_ins = ngram_set[g_i_1_pref_idx + 1:g_i_pref_idx + 1]
        for pref_gram, gram_pos in pref_ins:
            if pref_gram not in pref_ivx[t]:
                pref_ivx[t][pref_gram] = []
            pref_ivx[t][pref_gram].append((str_id, gram_pos))
            
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
for qword in qdb:
    query, t_ = qword.split(',')
    threshold = int(t_)
    qngrams = ngramify(query, n)
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
            # start = pref_len_map.get(qpref_gram, dict()).get(len(query) - threshold, 0)
            # end = pref_len_map.get(qpref_gram, dict()).get(len(query) + threshold + 1, len(pref_len_map.get(qpref_gram, dict()))) - 1
            for rec_id, cpos in listings:
                if global_order_sort(ngram_prefix_set[rec_id][-1][0]) > global_order_sort(pref_qngrams[-1][0]) \
                    and abs(cpos - qgram_pos) <= threshold:
                        candidate_set.add(rec_id)

    # then search by query pivots
    for qpiv_gram, qpiv_pos in query_pivots:
        for t in range(threshold + 1):
            if qpiv_gram not in pref_ivx[t]:
                continue
            
            listings = pref_ivx[t][qpiv_gram]
            # start = piv_len_map.get(qpiv_gram, dict()).get(len(query) - threshold, 0)
            # end = piv_len_map.get(qpiv_gram, dict()).get(len(query) + threshold + 1, len(piv_len_map.get(qpiv_gram, dict()))) - 1
            # for i in range(start, end + 1):
            for rec_id, cpos in listings:
                if global_order_sort(ngram_prefix_set[rec_id][-1][0]) <= global_order_sort(pref_qngrams[-1][0]) \
                    and abs(cpos - qpiv_pos) <= threshold:
                        candidate_set.add(rec_id)
                    
    print(f'Candidates for query {query} with threshold {threshold}:', len(candidate_set))
    for candidate in candidate_set:
        word = db[candidate]
        candidate_distance = ukkonen.distance(word, query, threshold + 1)
        # print('Candidate distance:', word, query, candidate_distance)
        if candidate_distance <= threshold:
            lve_sum += candidate


print('Result sum: ', lve_sum)
        

    # for rec in db:
    #     if ukkonen.distance(query, rec, threshold + 1) <= threshold:
    #         print(query, word, 'should match')