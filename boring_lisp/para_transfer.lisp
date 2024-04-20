
(load "./utils.lisp")

(defun /1024-with-times (time number)
  (if (= time 0)
      number
      (/ (/1024-with-times (- time 1) number) 1024)
      )
  )

(defun pt/Byte-to-GB (x)
  (/1024-with-times 3 x))

(defun pt/Byte-to-TB (x)
    (/1024-with-times 4 x)
  )

(defun pt/Parameter-to-f32-Byte (x)
  (/ (* x 32) 8)
  )

(defun pt/Parameter-to-f16-Byte (x)
  (/ (* x 16) 8)
  )

(defun pt/f32-Billion-to-GB (x)
  (pt/Byte-to-GB (pt/Parameter-to-f32-Byte
		  (* (float x) (expt 10 9))
		  ))
  )

(defun pt/f16-Billion-to-GB (x)
  (pt/Byte-to-GB (pt/Parameter-to-f16-Byte
		  (* (float x) (expt 10 9))
		  ))
  )

(defvar *DEVICE-MEMORY-MAP*
  '( ("4090" . 24)
    ("4080" . 16)
    ("3090" . 24)
    ("V100" . 32)
    ("A100" . 80)
    ))

(defun pt/compute-GPU-requirements (parameter-amount is-f32 GPU-type)
  (let* ((cal-func (if is-f32 'pt/f32-Billion-to-GB 'pt/f16-Billion-to-GB))
	 (GB-value (funcall cal-func parameter-amount))
	 (gpu-size (query-dict *DEVICE-MEMORY-MAP* GPU-type))
	 )
    (/ GB-value gpu-size))
  )

(pt/compute-GPU-requirements 7 t "4090")
;; (query-dict *DEVICE-MEMORY-MAP* "4090")
