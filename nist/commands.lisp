(flush-lines "^[ ]*$")
(flush_lines "\(°C\|hermo\)")
(set-buffer-file-coding-system utf-8-unix)

Need to remove the last entry from each line, as it is repeated



For temperatures below zero, we need to reverse the line

query-replace-regexp-region

\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\)\([ ]+-?[0-9]+\.[0-9]\{3\}\) \,(match-string 10)\9\8\7\6\5\4\3\2\1


Add commas

\([0-9]\)[ ]+ \1,
\([0-9]\)$ \1,
