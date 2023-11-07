##
## EPITECH PROJECT, 2023
## Crypto
## File description:
## Makefile
##

NAME = mypgp

all:
		cargo build
		mv target/debug/$(NAME) .

clean:
		cargo clean


fclean: clean
		rm -f $(NAME)

re: fclean all

.PHONY:	all clean fclean re