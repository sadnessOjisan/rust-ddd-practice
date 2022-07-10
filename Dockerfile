FROM mysql

EXPOSE 3306

ADD ./my.cnf /etc/mysql/conf.d/my.cnf


ENV MYSQL_ROOT_PASSWORD rust-ddd-practice
ENV MYSQL_ALLOW_EMPTY_PASSWORD local-pass
ENV MYSQL_RANDOM_ROOT_PASSWORD local-pass

CMD ["mysqld"]