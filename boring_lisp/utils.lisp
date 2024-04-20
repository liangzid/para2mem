




;;; for dict

; query
(defun query-dict (dict key)
  (let ((pair (assoc key dict :test #'equal)))
    (if pair
	(cdr pair)
	nil
	)
    )
  )
