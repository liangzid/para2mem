
(defun main()
  (format t "hello cl!~%")
  (sb-ext:quit)
  )


(sb-ext:save-lisp-and-die "hello-cl"
			  :toplevel #'main
			  :executable t)
