with recursive search_words(curr, next, end_sentinel) as (
    (select w.curr, w.next, w.end_sentinel
    from words w
    where start_sentinel = 't'
    order by random()
    limit 1)
    union all
    (select w.curr, w.next, w.end_sentinel
    from words w, search_words sw
    where w.curr = sw.next
    and sw.end_sentinel = 'f'
    order by random()
    limit 1)
)
select string_agg(curr, ' ') as txt from search_words;
