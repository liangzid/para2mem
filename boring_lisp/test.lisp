















(defmacro setq2 (x y z)
    `(progn
       (setq ,x ,z)
       (setq ,y ,z)
       )
  )

(setq2 x y 3)

