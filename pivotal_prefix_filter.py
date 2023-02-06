import ukkonen


def ngramify(doc: str, n: int) -> list[tuple[str, int]]:
    ngrams = []
    # padding to make sure we have enough ngrams
    # doc += '$' * (n - 1)
    ngram_count = len(doc) - n + 1
    for i in range(ngram_count):
        ngram = doc[i:i + n]
        ngrams.append((ngram , i))
    return ngrams


with open('./workloads/mini/input') as f:
    data = [l.strip() for l in f.readlines()]
    
srch_id = data.index('[SEARCH]')

db = [l for id, l in enumerate(data) if id < srch_id]
qdb = [l for id, l in enumerate(data) if id > srch_id]

occurence_map = dict()

# db = ['imyouteca', 'ubuntucom', 'utubbecou', 'youtbecom', 'yoytubeca']
# query = 'yotubecom'

pref_ivx = dict()
piv_ivx = dict()
pref_len_map = dict()
piv_len_map = dict()

threshold = 2
ngram_prefix_set = []

def global_order_sort(gram):
    return occurence_map.get(gram[0], [1, len(occurence_map.keys()) + 1])


def prefix_selection(ngrams, threshold):
    prefix_len = threshold * n + 1
    ngrams.sort(key=global_order_sort)
    return ngrams[:prefix_len]


def pivots_selection(prefix_set, threshold):
    pivots = []
    pivots.append(prefix_set[0])
    for ngram, ngram_pos in prefix_set[1:]:
        for pn in pivots:
            if abs(pn[1] - ngram_pos) < n:
                break
        else:
            pivots.append((ngram, ngram_pos))
            
        if len(pivots) == threshold + 1:
            break
    return pivots


db.sort(key=lambda x: len(x))

n = 2
ngram_db = [ngramify(rec, n) for rec in db]

for ngram_set in ngram_db:
    for ngram, ng_pos in ngram_set:
        if ngram not in occurence_map:
            occurence_map[ngram] = [0, len(occurence_map.keys()) + 1]
        occurence_map[ngram][0] += 1
        # occurence_map[ngram] += 1

for str_id, ngram_set in enumerate(ngram_db):
    ngram_prefix_set.append(prefix_selection(ngram_set, threshold))
    
    for pref_gram, gram_pos in ngram_prefix_set[-1]:
        if pref_gram not in pref_ivx:
            pref_ivx[pref_gram] = []
            pref_len_map[pref_gram] = dict()
            
        pref_ivx[pref_gram].append((str_id, gram_pos))
        
        if len(db[str_id]) not in pref_len_map[pref_gram]:
            pref_len_map[pref_gram][len(db[str_id])] = len(pref_ivx[pref_gram]) - 1
        
    
    print(ngram_prefix_set[-1])

# for qword in qdb[:-2]:
    # query, _ = qword.split(',')
    


# print('Query: ', pref_qngrams)

# for idx, ngrams in enumerate(ngram_prefix_set):
#     if len(set(ngram for ngram, _ in ngrams) & set(ngram for ngram, _ in pref_qngrams)) == 0:
#         print('Filtered', db[idx], ukkonen.distance(query, db[idx], threshold + 1))

# ngram_prefix_set = [ngrams for ngrams in ngram_prefix_set if len(set(ngrams) & set(pref_qngrams)) > 0]

pivot_ngrams = []
for str_id, ngram_set in enumerate(ngram_prefix_set):
    pivots = pivots_selection(ngram_set, threshold)
    
    for piv, piv_pos in pivots:
        if piv not in piv_ivx:
            piv_ivx[piv] = []
            piv_len_map[piv] = dict()
            
        piv_ivx[piv].append((str_id, piv_pos))
        
        if len(db[str_id]) not in piv_len_map[piv]:
            piv_len_map[piv][len(db[str_id])] = len(pref_ivx[piv]) - 1
    pivot_ngrams.append(pivots)
    # print(pivot_ngrams[-1])



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
    
            
for qword in qdb:
    threshold = 2
    n = 2
    query, _ = qword.split(',')
    qngrams = ngramify(query, n)
    pref_qngrams = prefix_selection(qngrams, threshold)
    query_pivots = pivots_selection(pref_qngrams, threshold)
    # Searching the indexes

    candidate_set = set()

    # first search by query prefixes
    for qpref_gram, qgram_pos in pref_qngrams:
        if qpref_gram not in pref_ivx:
            continue
        
        listings = pref_ivx[qpref_gram]
        start = pref_len_map.get(qpref_gram, dict()).get(len(query) - threshold, 0)
        end = pref_len_map.get(qpref_gram, dict()).get(len(query) + threshold + 1, len(pref_len_map.get(qpref_gram, dict()))) - 1
        for i in range(start, end + 1):
            rec_id, cpos = listings[i]
            if global_order_sort(ngram_prefix_set[rec_id][-1][0]) > global_order_sort(pref_qngrams[-1][0]) \
                and abs(cpos - qgram_pos) <= threshold:
                    candidate_set.add(rec_id)

    # then search by query pivots
    for qpiv_gram, qpiv_pos in query_pivots:
        if qpiv_gram not in piv_ivx:
            continue
        
        listings = piv_ivx[qpiv_gram]
        start = piv_len_map.get(qpiv_gram, dict()).get(len(query) - threshold, 0)
        end = piv_len_map.get(qpiv_gram, dict()).get(len(query) + threshold + 1, len(piv_len_map.get(qpiv_gram, dict()))) - 1
        for i in range(start, end + 1):
            rec_id, cpos = listings[i]
            if global_order_sort(ngram_prefix_set[rec_id][-1][0]) <= global_order_sort(pref_qngrams[-1][0]) \
                and abs(cpos - qpiv_pos) <= threshold:
                    candidate_set.add(rec_id)
                    
    print('Candidates:', len(candidate_set), '=>', candidate_set)
    for candidate in candidate_set:
        word = db[candidate]
        print('Candidate distance:', word, query, ukkonen.distance(word, query, threshold + 1))