#ifndef LIBDNF5_WRAPPER_H
#define LIBDNF5_WRAPPER_H

#include <stddef.h>

#ifdef HAVE_LIBDNF5
#include <libdnf5/base/base.hpp>
#include <libdnf5/repo/repo.hpp>
#include <libdnf5/rpm/package.hpp>
#include <libdnf5/rpm/package_query.hpp>
#include <libdnf5/rpm/transaction.hpp>
#else

typedef struct dnf5_base dnf5_base;
typedef struct dnf5_repo dnf5_repo;
typedef struct dnf5_package dnf5_package;
typedef struct dnf5_transaction dnf5_transaction;
typedef struct dnf5_query dnf5_query;

typedef enum {
    DNF5_OK = 0,
    DNF5_ERROR = 1,
    DNF5_ERROR_REPO = 2,
    DNF5_ERROR_PACKAGE = 3,
    DNF5_ERROR_TRANSACTION = 4,
} dnf5_error_t;

dnf5_base* dnf5_base_new(void);
void dnf5_base_free(dnf5_base* base);
int dnf5_base_setup(dnf5_base* base);
int dnf5_base_load_repos(dnf5_base* base);

dnf5_repo* dnf5_repo_new(dnf5_base* base, const char* id);
void dnf5_repo_free(dnf5_repo* repo);
int dnf5_repo_set_baseurl(dnf5_repo* repo, const char* url);
int dnf5_repo_enable(dnf5_repo* repo);
int dnf5_repo_load(dnf5_repo* repo);

dnf5_query* dnf5_query_new(dnf5_base* base);
void dnf5_query_free(dnf5_query* query);
int dnf5_query_filter_name(dnf5_query* query, const char* name);
int dnf5_query_filter_installed(dnf5_query* query, int installed);
size_t dnf5_query_size(dnf5_query* query);
dnf5_package* dnf5_query_get(dnf5_query* query, size_t index);

const char* dnf5_package_get_name(dnf5_package* pkg);
const char* dnf5_package_get_version(dnf5_package* pkg);
const char* dnf5_package_get_arch(dnf5_package* pkg);
const char* dnf5_package_get_summary(dnf5_package* pkg);
unsigned long long dnf5_package_get_download_size(dnf5_package* pkg);
unsigned long long dnf5_package_get_install_size(dnf5_package* pkg);

dnf5_transaction* dnf5_transaction_new(dnf5_base* base);
void dnf5_transaction_free(dnf5_transaction* trans);
int dnf5_transaction_add_install(dnf5_transaction* trans, dnf5_package* pkg);
int dnf5_transaction_add_remove(dnf5_transaction* trans, dnf5_package* pkg);
int dnf5_transaction_add_upgrade(dnf5_transaction* trans, dnf5_package* pkg);
int dnf5_transaction_resolve(dnf5_transaction* trans);
int dnf5_transaction_download(dnf5_transaction* trans);
int dnf5_transaction_test(dnf5_transaction* trans);
int dnf5_transaction_run(dnf5_transaction* trans);

#endif

#endif
