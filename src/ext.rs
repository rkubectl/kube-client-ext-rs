use super::*;

/// Extension trait for `kube::Client` that provides convenient methods for creating API clients
/// for common Kubernetes resources.
///
/// This trait extends the functionality of `kube::Client` by providing typed methods that return
/// `kube::Api<T>` instances for various Kubernetes resource types. It eliminates the need to
/// manually call `Api::all()`, `Api::namespaced()`, or `Api::default_namespaced()` for common
/// resources, making the code more readable and less verbose.
///
/// The trait includes methods for both cluster-scoped resources (like Nodes, ClusterRoles) and
/// namespace-scoped resources (like Pods, Services, Deployments). For namespace-scoped resources,
/// methods accept an optional namespace parameter - if `None` is provided, the default namespace
/// from the client configuration is used.
///
/// # Example
///
/// ```rust
/// use kube::Client;
/// use kube_client_ext::KubeClientApiExt;
///
/// # async fn example() -> kube::Result<()> {
/// let client = Client::try_default().await?;
///
/// // Get API clients for different resource types
/// let pods = client.pods(Some("my-namespace"));
/// let nodes = client.nodes();
/// let deployments = client.deployments(None); // Uses default namespace
/// # Ok(())
/// # }
/// ```
pub trait KubeClientApiExt: Clone {
    /// Returns an API client for managing APIServices in the cluster.
    ///
    /// APIServices are cluster-scoped resources that define additional APIs
    /// available on the API server.
    fn apiservices(&self) -> api::Api<apiregistrationv1::APIService> {
        self.api()
    }

    /// Returns an API client for managing ClusterRoles in the cluster.
    ///
    /// ClusterRoles are cluster-scoped RBAC resources that define a set of
    /// permissions that can be bound to subjects cluster-wide.
    fn clusterroles(&self) -> api::Api<rbacv1::ClusterRole> {
        self.api()
    }

    /// Returns an API client for managing ClusterRoleBindings in the cluster.
    ///
    /// ClusterRoleBindings bind ClusterRoles to subjects (users, groups, or
    /// service accounts) across the entire cluster.
    fn clusterrolebindings(&self) -> api::Api<rbacv1::ClusterRoleBinding> {
        self.api()
    }

    /// Returns an API client for managing CustomResourceDefinitions (CRDs).
    ///
    /// CRDs allow you to extend the Kubernetes API with custom resources
    /// specific to your applications.
    fn crds(&self) -> api::Api<apiextensionsv1::CustomResourceDefinition> {
        self.api()
    }

    /// Returns an API client for managing Endpoints in the specified namespace.
    ///
    /// Endpoints track the IP addresses and ports of pods that back a service.
    /// If no namespace is provided, uses the default namespace.
    fn endpoints<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Endpoints> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Nodes in the cluster.
    ///
    /// Nodes represent the worker machines in your cluster, whether virtual
    /// or physical machines.
    fn nodes(&self) -> api::Api<corev1::Node> {
        self.api()
    }

    /// Returns an API client for managing Namespaces in the cluster.
    ///
    /// Namespaces provide a scope for resource names and allow you to
    /// divide cluster resources between multiple users.
    fn namespaces(&self) -> api::Api<corev1::Namespace> {
        self.api()
    }

    /// Returns an API client for managing PersistentVolumes in the cluster.
    ///
    /// PersistentVolumes are cluster-scoped storage resources that exist
    /// independently of any individual pod.
    fn persistentvolumes(&self) -> api::Api<corev1::PersistentVolume> {
        self.api()
    }

    /// Returns an API client for managing StorageClasses in the cluster.
    ///
    /// StorageClasses describe different classes of storage available
    /// and how to provision PersistentVolumes.
    fn storageclasses(&self) -> api::Api<storagev1::StorageClass> {
        self.api()
    }

    /// Returns an API client for managing ConfigMaps in the specified namespace.
    ///
    /// ConfigMaps store non-confidential configuration data in key-value pairs.
    /// If no namespace is provided, uses the default namespace.
    fn configmaps<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::ConfigMap> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing DaemonSets in the specified namespace.
    ///
    /// DaemonSets ensure that all (or some) nodes run a copy of a specific pod.
    /// If no namespace is provided, uses the default namespace.
    fn daemonsets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<appsv1::DaemonSet> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Deployments in the specified namespace.
    ///
    /// Deployments provide declarative updates for pods and ReplicaSets.
    /// If no namespace is provided, uses the default namespace.
    fn deployments<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::Deployment> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing HorizontalPodAutoscalers in the specified namespace.
    ///
    /// HorizontalPodAutoscalers automatically scale the number of pods in a deployment,
    /// replica set, or stateful set based on observed metrics.
    /// If no namespace is provided, uses the default namespace.
    fn horizontalpodautoscalers<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<autoscalingv2::HorizontalPodAutoscaler> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Jobs in the specified namespace.
    ///
    /// Jobs run pods to completion and ensure that a specified number of
    /// pods successfully terminate. If no namespace is provided, uses the default namespace.
    fn jobs<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<batchv1::Job> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing CronJobs in the specified namespace.
    ///
    /// CronJobs create Jobs on a repeating schedule based on a cron expression.
    /// If no namespace is provided, uses the default namespace.
    fn cronjobs<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<batchv1::CronJob> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing PersistentVolumeClaims in the specified namespace.
    ///
    /// PersistentVolumeClaims are requests for storage by users that can be
    /// bound to PersistentVolumes. If no namespace is provided, uses the default namespace.
    fn persistentvolumeclaims<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<corev1::PersistentVolumeClaim> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing ReplicaSets in the specified namespace.
    ///
    /// ReplicaSets ensure that a specified number of pod replicas are running
    /// at any given time. If no namespace is provided, uses the default namespace.
    fn replicasets<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::ReplicaSet> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Roles in the specified namespace.
    ///
    /// Roles define a set of permissions within a particular namespace.
    /// If no namespace is provided, uses the default namespace.
    fn roles<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<rbacv1::Role> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing RoleBindings in the specified namespace.
    ///
    /// RoleBindings bind Roles or ClusterRoles to subjects within a specific namespace.
    /// If no namespace is provided, uses the default namespace.
    fn rolebindings<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<rbacv1::RoleBinding> {
        self.namespaced_k(namespace)
    }
    /// Returns an API client for managing Secrets in the specified namespace.
    ///
    /// Secrets store sensitive data such as passwords, OAuth tokens, and SSH keys.
    /// If no namespace is provided, uses the default namespace.
    fn secrets<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Secret> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Pods in the specified namespace.
    ///
    /// Pods are the smallest deployable units of computing that can be created
    /// and managed in Kubernetes. If no namespace is provided, uses the default namespace.
    fn pods<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Pod> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing ServiceAccounts in the specified namespace.
    ///
    /// ServiceAccounts provide an identity for processes that run in pods
    /// and are used for authentication to the Kubernetes API.
    /// If no namespace is provided, uses the default namespace.
    fn serviceaccounts<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<corev1::ServiceAccount> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing Services in the specified namespace.
    ///
    /// Services define a logical set of pods and a policy by which to access them,
    /// providing stable networking for dynamic pod sets.
    /// If no namespace is provided, uses the default namespace.
    fn services<'a>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<corev1::Service> {
        self.namespaced_k(namespace)
    }

    /// Returns an API client for managing StatefulSets in the specified namespace.
    ///
    /// StatefulSets manage the deployment and scaling of pods with persistent
    /// identities and stable network identities.
    /// If no namespace is provided, uses the default namespace.
    fn statefulsets<'a>(
        &self,
        namespace: impl Into<Option<&'a str>>,
    ) -> api::Api<appsv1::StatefulSet> {
        self.namespaced_k(namespace)
    }

    /// Creates a cluster-scoped API client for the specified Kubernetes resource type.
    ///
    /// This method is used internally by other methods to create API clients for
    /// cluster-scoped resources that are not bound to a specific namespace.
    fn api<K>(&self) -> api::Api<K>
    where
        K: client::Resource,
        <K as client::Resource>::DynamicType: Default;

    /// Creates a namespace-scoped API client using the default namespace.
    ///
    /// This method is used internally to create API clients for namespaced resources
    /// when no specific namespace is provided.
    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default;

    /// Creates a namespace-scoped API client for the specified namespace.
    ///
    /// This method is used internally to create API clients for namespaced resources
    /// within a specific namespace.
    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default;

    /// Creates a namespace-scoped API client, using either the provided namespace or the default.
    ///
    /// This is a utility method that conditionally creates either a specific namespaced
    /// API client or uses the default namespace depending on the input parameter.
    fn namespaced_k<'a, K>(&self, namespace: impl Into<Option<&'a str>>) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        let namespace = namespace.into();
        if let Some(namespace) = namespace {
            self.namespaced_api(namespace)
        } else {
            self.default_namespaced_api()
        }
    }
}

/// This is a helper trait that allows easy creation of common `kube::api` parameter objects
/// as a method on `kube::Client`, rather than free-standing functions. It doesn't actually
/// use the client itself, but it's convenient to have these methods available on the client.
pub trait KubeClientApiParamsExt: Clone {
    /// Creates default delete parameters with zero grace period.
    ///
    /// This will immediately terminate the resource without waiting
    /// for graceful shutdown.
    fn delete_params(&self) -> api::DeleteParams {
        api::DeleteParams::default().grace_period(0)
    }

    /// Creates delete parameters for background deletion with zero grace period.
    ///
    /// Background deletion means dependent objects will be deleted by the
    /// garbage collector after the owner is deleted.
    fn background_delete(&self) -> api::DeleteParams {
        api::DeleteParams::background().grace_period(0)
    }

    /// Creates delete parameters for foreground deletion with zero grace period.
    ///
    /// Foreground deletion means the owner will not be deleted until all
    /// dependent objects are deleted first.
    fn foreground_delete(&self) -> api::DeleteParams {
        api::DeleteParams::foreground().grace_period(0)
    }

    /// Creates delete parameters for orphan deletion with zero grace period.
    ///
    /// Orphan deletion means dependent objects will not be deleted and
    /// become orphaned when the owner is deleted.
    fn orphan_delete(&self) -> api::DeleteParams {
        api::DeleteParams::orphan().grace_period(0)
    }

    /// Creates default list parameters for resource listing operations.
    ///
    /// These parameters can be used to filter, sort, and paginate list operations.
    fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
    }

    /// Creates default watch parameters for resource watching operations.
    ///
    /// Watch parameters control how the watch stream behaves, including
    /// resource version and timeout settings.
    fn watch_params(&self) -> api::WatchParams {
        api::WatchParams::default()
    }

    /// Creates default post parameters for resource creation operations.
    ///
    /// Post parameters control how resources are created, including
    /// field manager information for server-side apply.
    fn post_params(&self) -> api::PostParams {
        api::PostParams::default()
    }

    /// Creates post parameters with a specific field manager.
    ///
    /// The field manager is used to identify which tool or user is managing
    /// specific fields in the resource for conflict resolution.
    fn post_params_with_manager(&self, manager: &str) -> api::PostParams {
        api::PostParams {
            field_manager: Some(manager.to_string()),
            ..self.post_params()
        }
    }

    /// Creates default patch parameters for resource update operations.
    ///
    /// Patch parameters control how resources are updated, including
    /// field manager information and force options.
    fn patch_params(&self) -> api::PatchParams {
        api::PatchParams::default()
    }

    /// Creates server-side apply patch parameters with the given field manager.
    ///
    /// Server-side apply allows declarative configuration management where
    /// the specified field manager takes ownership of the provided fields.
    fn patch_params_with_manager(&self, manager: &str) -> api::PatchParams {
        api::PatchParams::apply(manager)
    }

    /// Creates default log parameters for pod log retrieval operations.
    ///
    /// Log parameters control which logs are retrieved, including container
    /// selection, timestamp options, and follow behavior.
    fn log_params(&self) -> api::LogParams {
        api::LogParams::default()
    }

    /// Creates log parameters for retrieving logs from the previous container instance.
    ///
    /// This is useful when a container has been restarted and you want to see
    /// the logs from the previous run before the restart occurred.
    fn log_params_previous(&self) -> api::LogParams {
        api::LogParams {
            previous: true,
            ..api::LogParams::default()
        }
    }
}

impl KubeClientApiExt for client::Client {
    fn api<K>(&self) -> api::Api<K>
    where
        K: client::Resource,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::all(self.clone())
    }

    fn default_namespaced_api<K>(&self) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::default_namespaced(self.clone())
    }

    fn namespaced_api<K>(&self, namespace: &str) -> api::Api<K>
    where
        K: client::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as client::Resource>::DynamicType: Default,
    {
        api::Api::<K>::namespaced(self.clone(), namespace)
    }
}

impl KubeClientApiParamsExt for client::Client {}
