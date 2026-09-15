pub mod Microsoft {
    pub mod Management {
        pub mod Deployment {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AddPackageCatalogOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AddPackageCatalogOptions, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl AddPackageCatalogOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        AddPackageCatalogOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Name)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetName(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetName)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn SourceUri(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .SourceUri)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetSourceUri(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetSourceUri)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn Type(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Type)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetType(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetType)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn TrustLevel(
                    &self,
                ) -> windows_core::Result<PackageCatalogTrustLevel> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .TrustLevel)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetTrustLevel(
                    &self,
                    value: PackageCatalogTrustLevel,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetTrustLevel)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn CustomHeader(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CustomHeader)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetCustomHeader(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCustomHeader)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn Explicit(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Explicit)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetExplicit(&self, value: bool) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetExplicit)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn Priority(&self) -> windows_core::Result<i32> {
                    let this = &windows_core::Interface::cast::<
                        IAddPackageCatalogOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Priority)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPriority(&self, value: i32) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IAddPackageCatalogOptions2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetPriority)(windows_core::Interface::as_raw(this), value)
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for AddPackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IAddPackageCatalogOptions,
                >();
            }
            unsafe impl windows_core::Interface for AddPackageCatalogOptions {
                type Vtable = <IAddPackageCatalogOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAddPackageCatalogOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AddPackageCatalogOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.AddPackageCatalogOptions";
            }
            unsafe impl Send for AddPackageCatalogOptions {}
            unsafe impl Sync for AddPackageCatalogOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AddPackageCatalogResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AddPackageCatalogResult, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl AddPackageCatalogResult {
                pub fn Status(&self) -> windows_core::Result<AddPackageCatalogStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for AddPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IAddPackageCatalogResult,
                >();
            }
            unsafe impl windows_core::Interface for AddPackageCatalogResult {
                type Vtable = <IAddPackageCatalogResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAddPackageCatalogResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AddPackageCatalogResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.AddPackageCatalogResult";
            }
            unsafe impl Send for AddPackageCatalogResult {}
            unsafe impl Sync for AddPackageCatalogResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct AddPackageCatalogStatus(pub i32);
            impl AddPackageCatalogStatus {
                pub const Ok: Self = Self(0);
                pub const GroupPolicyError: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const InvalidOptions: Self = Self(4);
                pub const AccessDenied: Self = Self(5);
                pub const AuthenticationError: Self = Self(6);
            }
            impl windows_core::imp::TypeKind for AddPackageCatalogStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for AddPackageCatalogStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.AddPackageCatalogStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.AddPackageCatalogStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AuthenticationArguments(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AuthenticationArguments, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl AuthenticationArguments {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        AuthenticationArguments,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn AuthenticationMode(
                    &self,
                ) -> windows_core::Result<AuthenticationMode> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AuthenticationMode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAuthenticationMode(
                    &self,
                    value: AuthenticationMode,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAuthenticationMode)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationAccount(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AuthenticationAccount)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetAuthenticationAccount(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAuthenticationAccount)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for AuthenticationArguments {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IAuthenticationArguments,
                >();
            }
            unsafe impl windows_core::Interface for AuthenticationArguments {
                type Vtable = <IAuthenticationArguments as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAuthenticationArguments as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AuthenticationArguments {
                const NAME: &'static str = "Microsoft.Management.Deployment.AuthenticationArguments";
            }
            unsafe impl Send for AuthenticationArguments {}
            unsafe impl Sync for AuthenticationArguments {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AuthenticationInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AuthenticationInfo, windows_core::IUnknown, windows_core::IInspectable
            );
            impl AuthenticationInfo {
                pub fn AuthenticationType(
                    &self,
                ) -> windows_core::Result<AuthenticationType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AuthenticationType)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn MicrosoftEntraIdAuthenticationInfo(
                    &self,
                ) -> windows_core::Result<MicrosoftEntraIdAuthenticationInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .MicrosoftEntraIdAuthenticationInfo)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for AuthenticationInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IAuthenticationInfo,
                >();
            }
            unsafe impl windows_core::Interface for AuthenticationInfo {
                type Vtable = <IAuthenticationInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAuthenticationInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AuthenticationInfo {
                const NAME: &'static str = "Microsoft.Management.Deployment.AuthenticationInfo";
            }
            unsafe impl Send for AuthenticationInfo {}
            unsafe impl Sync for AuthenticationInfo {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct AuthenticationMode(pub i32);
            impl AuthenticationMode {
                pub const Interactive: Self = Self(0);
                pub const SilentPreferred: Self = Self(1);
                pub const Silent: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for AuthenticationMode {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for AuthenticationMode {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.AuthenticationMode;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.AuthenticationMode",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct AuthenticationType(pub i32);
            impl AuthenticationType {
                pub const Unknown: Self = Self(0);
                pub const None: Self = Self(1);
                pub const MicrosoftEntraId: Self = Self(2);
                pub const MicrosoftEntraIdForAzureBlobStorage: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for AuthenticationType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for AuthenticationType {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.AuthenticationType;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.AuthenticationType",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct CatalogPackage(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                CatalogPackage, windows_core::IUnknown, windows_core::IInspectable
            );
            impl CatalogPackage {
                pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Id)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Name)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn InstalledVersion(
                    &self,
                ) -> windows_core::Result<PackageVersionInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstalledVersion)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn AvailableVersions(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<PackageVersionId>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AvailableVersions)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn DefaultInstallVersion(
                    &self,
                ) -> windows_core::Result<PackageVersionInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .DefaultInstallVersion)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetPackageVersionInfo<P0>(
                    &self,
                    versionkey: P0,
                ) -> windows_core::Result<PackageVersionInfo>
                where
                    P0: windows_core::Param<PackageVersionId>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetPackageVersionInfo)(
                                windows_core::Interface::as_raw(self),
                                versionkey.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn IsUpdateAvailable(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .IsUpdateAvailable)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn CheckInstalledStatusAsync(
                    &self,
                    checktypes: InstalledStatusType,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperation<CheckInstalledStatusResult>,
                > {
                    let this = &windows_core::Interface::cast::<ICatalogPackage2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CheckInstalledStatusAsync)(
                                windows_core::Interface::as_raw(this),
                                checktypes,
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CheckInstalledStatus(
                    &self,
                    checktypes: InstalledStatusType,
                ) -> windows_core::Result<CheckInstalledStatusResult> {
                    let this = &windows_core::Interface::cast::<ICatalogPackage2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CheckInstalledStatus)(
                                windows_core::Interface::as_raw(this),
                                checktypes,
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CheckInstalledStatusAsync2(
                    &self,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperation<CheckInstalledStatusResult>,
                > {
                    let this = &windows_core::Interface::cast::<ICatalogPackage2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CheckInstalledStatusAsync2)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CheckInstalledStatus2(
                    &self,
                ) -> windows_core::Result<CheckInstalledStatusResult> {
                    let this = &windows_core::Interface::cast::<ICatalogPackage2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CheckInstalledStatus2)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CatalogPriority(&self) -> windows_core::Result<i32> {
                    let this = &windows_core::Interface::cast::<ICatalogPackage3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CatalogPriority)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                            .and_then(|r__: windows_reference::IReference<i32>| {
                                r__.Value()
                            })
                    }
                }
            }
            impl windows_core::RuntimeType for CatalogPackage {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    ICatalogPackage,
                >();
            }
            unsafe impl windows_core::Interface for CatalogPackage {
                type Vtable = <ICatalogPackage as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ICatalogPackage as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for CatalogPackage {
                const NAME: &'static str = "Microsoft.Management.Deployment.CatalogPackage";
            }
            unsafe impl Send for CatalogPackage {}
            unsafe impl Sync for CatalogPackage {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct CatalogPackageMetadata(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                CatalogPackageMetadata, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl CatalogPackageMetadata {
                pub fn Locale(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Locale)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Publisher(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Publisher)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PublisherUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PublisherUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PublisherSupportUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PublisherSupportUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PrivacyUrl(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PrivacyUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Author(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Author)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PackageName(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageName)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PackageUrl(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn License(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .License)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn LicenseUrl(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .LicenseUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Copyright(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Copyright)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn CopyrightUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CopyrightUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn ShortDescription(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ShortDescription)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Description(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Description)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Tags(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<windows_core::HSTRING>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Tags)(windows_core::Interface::as_raw(self), &mut result__)
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Agreements(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<PackageAgreement>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Agreements)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Documentations(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<Documentation>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Documentations)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Icons(
                    &self,
                ) -> windows_core::Result<windows_collections::IVectorView<Icon>> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Icons)(windows_core::Interface::as_raw(self), &mut result__)
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ReleaseNotes(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ReleaseNotes)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn ReleaseNotesUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ReleaseNotesUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PurchaseUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PurchaseUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn InstallationNotes(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallationNotes)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for CatalogPackageMetadata {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    ICatalogPackageMetadata,
                >();
            }
            unsafe impl windows_core::Interface for CatalogPackageMetadata {
                type Vtable = <ICatalogPackageMetadata as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ICatalogPackageMetadata as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for CatalogPackageMetadata {
                const NAME: &'static str = "Microsoft.Management.Deployment.CatalogPackageMetadata";
            }
            unsafe impl Send for CatalogPackageMetadata {}
            unsafe impl Sync for CatalogPackageMetadata {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct CheckInstalledStatusResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                CheckInstalledStatusResult, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl CheckInstalledStatusResult {
                pub fn Status(
                    &self,
                ) -> windows_core::Result<CheckInstalledStatusResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn PackageInstalledStatus(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<PackageInstallerInstalledStatus>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageInstalledStatus)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for CheckInstalledStatusResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    ICheckInstalledStatusResult,
                >();
            }
            unsafe impl windows_core::Interface for CheckInstalledStatusResult {
                type Vtable = <ICheckInstalledStatusResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ICheckInstalledStatusResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for CheckInstalledStatusResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.CheckInstalledStatusResult";
            }
            unsafe impl Send for CheckInstalledStatusResult {}
            unsafe impl Sync for CheckInstalledStatusResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct CheckInstalledStatusResultStatus(pub i32);
            impl CheckInstalledStatusResultStatus {
                pub const Ok: Self = Self(0);
                pub const InternalError: Self = Self(1);
            }
            impl windows_core::imp::TypeKind for CheckInstalledStatusResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for CheckInstalledStatusResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.CheckInstalledStatusResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.CheckInstalledStatusResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct CompareResult(pub i32);
            impl CompareResult {
                pub const Unknown: Self = Self(0);
                pub const Lesser: Self = Self(1);
                pub const Equal: Self = Self(2);
                pub const Greater: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for CompareResult {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for CompareResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.CompareResult;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.CompareResult",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct CompositeSearchBehavior(pub i32);
            impl CompositeSearchBehavior {
                pub const LocalCatalogs: Self = Self(0);
                pub const RemotePackagesFromRemoteCatalogs: Self = Self(1);
                pub const RemotePackagesFromAllCatalogs: Self = Self(2);
                pub const AllCatalogs: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for CompositeSearchBehavior {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for CompositeSearchBehavior {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.CompositeSearchBehavior;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.CompositeSearchBehavior",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ConnectResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ConnectResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl ConnectResult {
                pub fn Status(&self) -> windows_core::Result<ConnectResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn PackageCatalog(&self) -> windows_core::Result<PackageCatalog> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageCatalog)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    let this = &windows_core::Interface::cast::<IConnectResult2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for ConnectResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IConnectResult,
                >();
            }
            unsafe impl windows_core::Interface for ConnectResult {
                type Vtable = <IConnectResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IConnectResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ConnectResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.ConnectResult";
            }
            unsafe impl Send for ConnectResult {}
            unsafe impl Sync for ConnectResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct ConnectResultStatus(pub i32);
            impl ConnectResultStatus {
                pub const Ok: Self = Self(0);
                pub const CatalogError: Self = Self(1);
                pub const SourceAgreementsNotAccepted: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for ConnectResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for ConnectResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.ConnectResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ConnectResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct CreateCompositePackageCatalogOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                CreateCompositePackageCatalogOptions, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl CreateCompositePackageCatalogOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        CreateCompositePackageCatalogOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Catalogs(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVector<PackageCatalogReference>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Catalogs)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CompositeSearchBehavior(
                    &self,
                ) -> windows_core::Result<CompositeSearchBehavior> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CompositeSearchBehavior)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetCompositeSearchBehavior(
                    &self,
                    value: CompositeSearchBehavior,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCompositeSearchBehavior)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn InstalledScope(
                    &self,
                ) -> windows_core::Result<PackageInstallScope> {
                    let this = &windows_core::Interface::cast::<
                        ICreateCompositePackageCatalogOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .InstalledScope)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetInstalledScope(
                    &self,
                    value: PackageInstallScope,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        ICreateCompositePackageCatalogOptions2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetInstalledScope)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for CreateCompositePackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    ICreateCompositePackageCatalogOptions,
                >();
            }
            unsafe impl windows_core::Interface
            for CreateCompositePackageCatalogOptions {
                type Vtable = <ICreateCompositePackageCatalogOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ICreateCompositePackageCatalogOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for CreateCompositePackageCatalogOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.CreateCompositePackageCatalogOptions";
            }
            unsafe impl Send for CreateCompositePackageCatalogOptions {}
            unsafe impl Sync for CreateCompositePackageCatalogOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Documentation(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                Documentation, windows_core::IUnknown, windows_core::IInspectable
            );
            impl Documentation {
                pub fn DocumentLabel(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .DocumentLabel)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn DocumentUrl(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .DocumentUrl)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for Documentation {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IDocumentation,
                >();
            }
            unsafe impl windows_core::Interface for Documentation {
                type Vtable = <IDocumentation as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDocumentation as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Documentation {
                const NAME: &'static str = "Microsoft.Management.Deployment.Documentation";
            }
            unsafe impl Send for Documentation {}
            unsafe impl Sync for Documentation {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DownloadOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DownloadOptions, windows_core::IUnknown, windows_core::IInspectable
            );
            impl DownloadOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DownloadOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn PackageVersionId(
                    &self,
                ) -> windows_core::Result<PackageVersionId> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetPackageVersionId<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<PackageVersionId>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn InstallerType(
                    &self,
                ) -> windows_core::Result<PackageInstallerType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallerType)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetInstallerType(
                    &self,
                    value: PackageInstallerType,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetInstallerType)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn Scope(&self) -> windows_core::Result<PackageInstallScope> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Scope)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetScope(
                    &self,
                    value: PackageInstallScope,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetScope)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn Architecture(
                    &self,
                ) -> windows_core::Result<
                    super::super::super::Windows::System::ProcessorArchitecture,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Architecture)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetArchitecture(
                    &self,
                    value: super::super::super::Windows::System::ProcessorArchitecture,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetArchitecture)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn Locale(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Locale)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetLocale(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetLocale)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn DownloadDirectory(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .DownloadDirectory)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetDownloadDirectory(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetDownloadDirectory)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AllowHashMismatch(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAllowHashMismatch(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn SkipDependencies(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .SkipDependencies)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetSkipDependencies(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetSkipDependencies)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AcceptPackageAgreements(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AcceptPackageAgreements)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAcceptPackageAgreements(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAcceptPackageAgreements)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetCorrelationData(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCorrelationData)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationArguments(
                    &self,
                ) -> windows_core::Result<AuthenticationArguments> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetAuthenticationArguments<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<AuthenticationArguments>,
                {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn SkipMicrosoftStoreLicense(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .SkipMicrosoftStoreLicense)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetSkipMicrosoftStoreLicense(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetSkipMicrosoftStoreLicense)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn Platform(&self) -> windows_core::Result<WindowsPlatform> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Platform)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPlatform(
                    &self,
                    value: WindowsPlatform,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetPlatform)(windows_core::Interface::as_raw(this), value)
                            .ok()
                    }
                }
                pub fn TargetOSVersion(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .TargetOSVersion)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetTargetOSVersion(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IDownloadOptions3,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetTargetOSVersion)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for DownloadOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IDownloadOptions,
                >();
            }
            unsafe impl windows_core::Interface for DownloadOptions {
                type Vtable = <IDownloadOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDownloadOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DownloadOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.DownloadOptions";
            }
            unsafe impl Send for DownloadOptions {}
            unsafe impl Sync for DownloadOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DownloadResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DownloadResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl DownloadResult {
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Status(&self) -> windows_core::Result<DownloadResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for DownloadResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IDownloadResult,
                >();
            }
            unsafe impl windows_core::Interface for DownloadResult {
                type Vtable = <IDownloadResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDownloadResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DownloadResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.DownloadResult";
            }
            unsafe impl Send for DownloadResult {}
            unsafe impl Sync for DownloadResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct DownloadResultStatus(pub i32);
            impl DownloadResultStatus {
                pub const Ok: Self = Self(0);
                pub const BlockedByPolicy: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const InvalidOptions: Self = Self(4);
                pub const DownloadError: Self = Self(5);
                pub const ManifestError: Self = Self(6);
                pub const NoApplicableInstallers: Self = Self(7);
                pub const PackageAgreementsNotAccepted: Self = Self(8);
            }
            impl windows_core::imp::TypeKind for DownloadResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for DownloadResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.DownloadResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.DownloadResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct EditPackageCatalogOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                EditPackageCatalogOptions, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl EditPackageCatalogOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        EditPackageCatalogOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Name)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetName(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetName)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn Explicit(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Explicit)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                            .and_then(|r__: windows_reference::IReference<bool>| {
                                r__.Value()
                            })
                    }
                }
                pub fn SetExplicit(
                    &self,
                    value: Option<bool>,
                ) -> windows_core::Result<()> {
                    let value__ = value
                        .map(<windows_reference::IReference<bool> as From<_>>::from);
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetExplicit)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Param::param(value__.as_ref()).abi(),
                            )
                            .ok()
                    }
                }
                pub fn Priority(&self) -> windows_core::Result<i32> {
                    let this = &windows_core::Interface::cast::<
                        IEditPackageCatalogOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Priority)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                            .and_then(|r__: windows_reference::IReference<i32>| {
                                r__.Value()
                            })
                    }
                }
                pub fn SetPriority(
                    &self,
                    value: Option<i32>,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IEditPackageCatalogOptions2,
                    >(self)?;
                    let value__ = value
                        .map(<windows_reference::IReference<i32> as From<_>>::from);
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetPriority)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Param::param(value__.as_ref()).abi(),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for EditPackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IEditPackageCatalogOptions,
                >();
            }
            unsafe impl windows_core::Interface for EditPackageCatalogOptions {
                type Vtable = <IEditPackageCatalogOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IEditPackageCatalogOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for EditPackageCatalogOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.EditPackageCatalogOptions";
            }
            unsafe impl Send for EditPackageCatalogOptions {}
            unsafe impl Sync for EditPackageCatalogOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct EditPackageCatalogResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                EditPackageCatalogResult, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl EditPackageCatalogResult {
                pub fn Status(&self) -> windows_core::Result<EditPackageCatalogStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for EditPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IEditPackageCatalogResult,
                >();
            }
            unsafe impl windows_core::Interface for EditPackageCatalogResult {
                type Vtable = <IEditPackageCatalogResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IEditPackageCatalogResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for EditPackageCatalogResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.EditPackageCatalogResult";
            }
            unsafe impl Send for EditPackageCatalogResult {}
            unsafe impl Sync for EditPackageCatalogResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct EditPackageCatalogStatus(pub i32);
            impl EditPackageCatalogStatus {
                pub const Ok: Self = Self(0);
                pub const GroupPolicyError: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const AccessDenied: Self = Self(4);
                pub const InvalidOptions: Self = Self(5);
            }
            impl windows_core::imp::TypeKind for EditPackageCatalogStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for EditPackageCatalogStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.EditPackageCatalogStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.EditPackageCatalogStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct ElevationRequirement(pub i32);
            impl ElevationRequirement {
                pub const Unknown: Self = Self(0);
                pub const ElevationRequired: Self = Self(1);
                pub const ElevationProhibited: Self = Self(2);
                pub const ElevatesSelf: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for ElevationRequirement {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for ElevationRequirement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.ElevationRequirement;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ElevationRequirement",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct FindPackagesOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                FindPackagesOptions, windows_core::IUnknown, windows_core::IInspectable
            );
            impl FindPackagesOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        FindPackagesOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Selectors(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVector<PackageMatchFilter>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Selectors)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Filters(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVector<PackageMatchFilter>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Filters)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ResultLimit(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ResultLimit)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetResultLimit(&self, value: u32) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetResultLimit)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for FindPackagesOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IFindPackagesOptions,
                >();
            }
            unsafe impl windows_core::Interface for FindPackagesOptions {
                type Vtable = <IFindPackagesOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IFindPackagesOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for FindPackagesOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.FindPackagesOptions";
            }
            unsafe impl Send for FindPackagesOptions {}
            unsafe impl Sync for FindPackagesOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct FindPackagesResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                FindPackagesResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl FindPackagesResult {
                pub fn Status(&self) -> windows_core::Result<FindPackagesResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Matches(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<MatchResult>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Matches)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn WasLimitExceeded(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .WasLimitExceeded)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    let this = &windows_core::Interface::cast::<
                        IFindPackagesResult2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for FindPackagesResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IFindPackagesResult,
                >();
            }
            unsafe impl windows_core::Interface for FindPackagesResult {
                type Vtable = <IFindPackagesResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IFindPackagesResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for FindPackagesResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.FindPackagesResult";
            }
            unsafe impl Send for FindPackagesResult {}
            unsafe impl Sync for FindPackagesResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct FindPackagesResultStatus(pub i32);
            impl FindPackagesResultStatus {
                pub const Ok: Self = Self(0);
                pub const BlockedByPolicy: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const InvalidOptions: Self = Self(4);
                pub const AuthenticationError: Self = Self(5);
                pub const AccessDenied: Self = Self(6);
            }
            impl windows_core::imp::TypeKind for FindPackagesResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for FindPackagesResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.FindPackagesResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.FindPackagesResultStatus",
                );
            }
            windows_core::imp::define_interface!(
                IAddPackageCatalogOptions, IAddPackageCatalogOptions_Vtbl,
                0xc842f9da_e721_5cdc_938d_4ee84b1eaf68
            );
            impl windows_core::RuntimeType for IAddPackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IAddPackageCatalogOptions",
                );
            }
            #[repr(C)]
            pub struct IAddPackageCatalogOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Name: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SourceUri: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetSourceUri: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Type: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub TrustLevel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageCatalogTrustLevel,
                ) -> windows_core::HRESULT,
                pub SetTrustLevel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageCatalogTrustLevel,
                ) -> windows_core::HRESULT,
                pub CustomHeader: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetCustomHeader: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Explicit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetExplicit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAddPackageCatalogOptions2, IAddPackageCatalogOptions2_Vtbl,
                0xeb07d729_b65e_5df0_ba9d_05dc506fb893
            );
            impl windows_core::RuntimeType for IAddPackageCatalogOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IAddPackageCatalogOptions2",
                );
            }
            #[repr(C)]
            pub struct IAddPackageCatalogOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Priority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut i32,
                ) -> windows_core::HRESULT,
                pub SetPriority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    i32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAddPackageCatalogResult, IAddPackageCatalogResult_Vtbl,
                0x0119c87a_cf30_5208_a204_2c5339d3063f
            );
            impl windows_core::RuntimeType for IAddPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IAddPackageCatalogResult",
                );
            }
            #[repr(C)]
            pub struct IAddPackageCatalogResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut AddPackageCatalogStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAuthenticationArguments, IAuthenticationArguments_Vtbl,
                0xe30ab203_7df5_5b44_9218_2ee866f241a0
            );
            impl windows_core::RuntimeType for IAuthenticationArguments {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IAuthenticationArguments",
                );
            }
            #[repr(C)]
            pub struct IAuthenticationArguments_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut AuthenticationMode,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    AuthenticationMode,
                ) -> windows_core::HRESULT,
                pub AuthenticationAccount: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationAccount: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAuthenticationInfo, IAuthenticationInfo_Vtbl,
                0x986a1a9d_84e6_56dc_b195_3b640dd4470a
            );
            impl windows_core::RuntimeType for IAuthenticationInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IAuthenticationInfo",
                );
            }
            #[repr(C)]
            pub struct IAuthenticationInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut AuthenticationType,
                ) -> windows_core::HRESULT,
                pub MicrosoftEntraIdAuthenticationInfo: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICatalogPackage, ICatalogPackage_Vtbl,
                0x7aceab52_4735_53d5_8fd1_8cf7f64838dc
            );
            impl windows_core::RuntimeType for ICatalogPackage {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICatalogPackage",
                );
            }
            #[repr(C)]
            pub struct ICatalogPackage_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Id: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Name: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub InstalledVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AvailableVersions: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub DefaultInstallVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetPackageVersionInfo: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub IsUpdateAvailable: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICatalogPackage2, ICatalogPackage2_Vtbl,
                0x814e6641_24fd_5ae0_a2e1_e0c2dc407e1c
            );
            impl windows_core::RuntimeType for ICatalogPackage2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICatalogPackage2",
                );
            }
            #[repr(C)]
            pub struct ICatalogPackage2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CheckInstalledStatusAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    InstalledStatusType,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CheckInstalledStatus: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    InstalledStatusType,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CheckInstalledStatusAsync2: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CheckInstalledStatus2: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICatalogPackage3, ICatalogPackage3_Vtbl,
                0x80e88b39_088d_59a7_b168_0b5adff09c4d
            );
            impl windows_core::RuntimeType for ICatalogPackage3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICatalogPackage3",
                );
            }
            #[repr(C)]
            pub struct ICatalogPackage3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CatalogPriority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICatalogPackageMetadata, ICatalogPackageMetadata_Vtbl,
                0xffd4e24f_5add_5728_a9c0_d8803622a753
            );
            impl windows_core::RuntimeType for ICatalogPackageMetadata {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICatalogPackageMetadata",
                );
            }
            #[repr(C)]
            pub struct ICatalogPackageMetadata_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Locale: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Publisher: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PublisherUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PublisherSupportUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PrivacyUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Author: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub License: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub LicenseUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Copyright: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CopyrightUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ShortDescription: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Description: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Tags: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Agreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Documentations: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Icons: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ReleaseNotes: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ReleaseNotesUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PurchaseUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub InstallationNotes: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICheckInstalledStatusResult, ICheckInstalledStatusResult_Vtbl,
                0x0d7f2e40_647a_5996_ac36_a0786751dd4f
            );
            impl windows_core::RuntimeType for ICheckInstalledStatusResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICheckInstalledStatusResult",
                );
            }
            #[repr(C)]
            pub struct ICheckInstalledStatusResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut CheckInstalledStatusResultStatus,
                ) -> windows_core::HRESULT,
                pub PackageInstalledStatus: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IConnectResult, IConnectResult_Vtbl,
                0x7e9bd693_9a94_5ec9_8eaf_70a8156b873d
            );
            impl windows_core::RuntimeType for IConnectResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IConnectResult",
                );
            }
            #[repr(C)]
            pub struct IConnectResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut ConnectResultStatus,
                ) -> windows_core::HRESULT,
                pub PackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IConnectResult2, IConnectResult2_Vtbl,
                0xabaf2609_ee22_5bb0_b1e4_a390eec631fc
            );
            impl windows_core::RuntimeType for IConnectResult2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IConnectResult2",
                );
            }
            #[repr(C)]
            pub struct IConnectResult2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICreateCompositePackageCatalogOptions,
                ICreateCompositePackageCatalogOptions_Vtbl,
                0x21abaa76_089d_51c5_a745_c85eefe70116
            );
            impl windows_core::RuntimeType for ICreateCompositePackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICreateCompositePackageCatalogOptions",
                );
            }
            #[repr(C)]
            pub struct ICreateCompositePackageCatalogOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Catalogs: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CompositeSearchBehavior: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut CompositeSearchBehavior,
                ) -> windows_core::HRESULT,
                pub SetCompositeSearchBehavior: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    CompositeSearchBehavior,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ICreateCompositePackageCatalogOptions2,
                ICreateCompositePackageCatalogOptions2_Vtbl,
                0x8c8fd1bd_8f72_5863_9495_ef2cd5bc823c
            );
            impl windows_core::RuntimeType for ICreateCompositePackageCatalogOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ICreateCompositePackageCatalogOptions2",
                );
            }
            #[repr(C)]
            pub struct ICreateCompositePackageCatalogOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InstalledScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallScope,
                ) -> windows_core::HRESULT,
                pub SetInstalledScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallScope,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDocumentation, IDocumentation_Vtbl,
                0x2e8d35dd_7677_583c_836d_ff52ebbff39a
            );
            impl windows_core::RuntimeType for IDocumentation {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IDocumentation",
                );
            }
            #[repr(C)]
            pub struct IDocumentation_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub DocumentLabel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub DocumentUrl: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDownloadOptions, IDownloadOptions_Vtbl,
                0x94c92c4b_43f5_5ca3_bbbe_9f432c9546bc
            );
            impl windows_core::RuntimeType for IDownloadOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IDownloadOptions",
                );
            }
            #[repr(C)]
            pub struct IDownloadOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub InstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallerType,
                ) -> windows_core::HRESULT,
                pub SetInstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallerType,
                ) -> windows_core::HRESULT,
                pub Scope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallScope,
                ) -> windows_core::HRESULT,
                pub SetScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallScope,
                ) -> windows_core::HRESULT,
                pub Architecture: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut super::super::super::Windows::System::ProcessorArchitecture,
                ) -> windows_core::HRESULT,
                pub SetArchitecture: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    super::super::super::Windows::System::ProcessorArchitecture,
                ) -> windows_core::HRESULT,
                pub Locale: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetLocale: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub DownloadDirectory: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetDownloadDirectory: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub SkipDependencies: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetSkipDependencies: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub AcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetCorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDownloadOptions2, IDownloadOptions2_Vtbl,
                0x3d2bfa6c_892b_563f_8433_cd42c1d50d1c
            );
            impl windows_core::RuntimeType for IDownloadOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IDownloadOptions2",
                );
            }
            #[repr(C)]
            pub struct IDownloadOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDownloadOptions3, IDownloadOptions3_Vtbl,
                0xafd0d76c_0c4b_5413_a362_797d95596017
            );
            impl windows_core::RuntimeType for IDownloadOptions3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IDownloadOptions3",
                );
            }
            #[repr(C)]
            pub struct IDownloadOptions3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SkipMicrosoftStoreLicense: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetSkipMicrosoftStoreLicense: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub Platform: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut WindowsPlatform,
                ) -> windows_core::HRESULT,
                pub SetPlatform: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    WindowsPlatform,
                ) -> windows_core::HRESULT,
                pub TargetOSVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetTargetOSVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDownloadResult, IDownloadResult_Vtbl,
                0x024ec7bb_da1a_5d15_b4d6_a33253269260
            );
            impl windows_core::RuntimeType for IDownloadResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IDownloadResult",
                );
            }
            #[repr(C)]
            pub struct IDownloadResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut DownloadResultStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IEditPackageCatalogOptions, IEditPackageCatalogOptions_Vtbl,
                0x81f2b672_266b_53c5_aa49_80eebbde344b
            );
            impl windows_core::RuntimeType for IEditPackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IEditPackageCatalogOptions",
                );
            }
            #[repr(C)]
            pub struct IEditPackageCatalogOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Name: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Explicit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetExplicit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IEditPackageCatalogOptions2, IEditPackageCatalogOptions2_Vtbl,
                0x571683fd_4f9e_501c_a460_d00cbb0d39a8
            );
            impl windows_core::RuntimeType for IEditPackageCatalogOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IEditPackageCatalogOptions2",
                );
            }
            #[repr(C)]
            pub struct IEditPackageCatalogOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Priority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPriority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IEditPackageCatalogResult, IEditPackageCatalogResult_Vtbl,
                0xbc279bbb_92d2_5e34_96fe_d377396ad12a
            );
            impl windows_core::RuntimeType for IEditPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IEditPackageCatalogResult",
                );
            }
            #[repr(C)]
            pub struct IEditPackageCatalogResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut EditPackageCatalogStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFindPackagesOptions, IFindPackagesOptions_Vtbl,
                0xa5270edd_7da7_57a3_bace_f2593553561f
            );
            impl windows_core::RuntimeType for IFindPackagesOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IFindPackagesOptions",
                );
            }
            #[repr(C)]
            pub struct IFindPackagesOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Selectors: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Filters: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ResultLimit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
                pub SetResultLimit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    u32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFindPackagesResult, IFindPackagesResult_Vtbl,
                0x8760dc7a_a46b_5511_9986_826f8b0a0941
            );
            impl windows_core::RuntimeType for IFindPackagesResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IFindPackagesResult",
                );
            }
            #[repr(C)]
            pub struct IFindPackagesResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut FindPackagesResultStatus,
                ) -> windows_core::HRESULT,
                pub Matches: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub WasLimitExceeded: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFindPackagesResult2, IFindPackagesResult2_Vtbl,
                0xfc45b470_7290_5a4b_925f_52b1d73fc58b
            );
            impl windows_core::RuntimeType for IFindPackagesResult2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IFindPackagesResult2",
                );
            }
            #[repr(C)]
            pub struct IFindPackagesResult2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IIcon, IIcon_Vtbl, 0x90d54c45_4f49_5ce7_8337_1f150d853a32
            );
            impl windows_core::RuntimeType for IIcon {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IIcon",
                );
            }
            #[repr(C)]
            pub struct IIcon_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Url: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub FileType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut IconFileType,
                ) -> windows_core::HRESULT,
                pub Resolution: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut IconResolution,
                ) -> windows_core::HRESULT,
                pub Theme: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut IconTheme,
                ) -> windows_core::HRESULT,
                pub Sha256: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                    *mut *mut u8,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions, IInstallOptions_Vtbl,
                0x6ee9db69_ab48_5e72_a474_33a924cd23b3
            );
            impl windows_core::RuntimeType for IInstallOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PreferredInstallLocation: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPreferredInstallLocation: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageInstallScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallScope,
                ) -> windows_core::HRESULT,
                pub SetPackageInstallScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallScope,
                ) -> windows_core::HRESULT,
                pub PackageInstallMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallMode,
                ) -> windows_core::HRESULT,
                pub SetPackageInstallMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallMode,
                ) -> windows_core::HRESULT,
                pub LogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetLogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub ReplacementInstallerArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetReplacementInstallerArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetCorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AdditionalPackageCatalogArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAdditionalPackageCatalogArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions2, IInstallOptions2_Vtbl,
                0x8ebba822_fea1_583c_9d3d_a5d88750ce62
            );
            impl windows_core::RuntimeType for IInstallOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions2",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AllowedArchitectures: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions3, IInstallOptions3_Vtbl,
                0x95c289ad_5c59_52cc_aa06_383274c98440
            );
            impl windows_core::RuntimeType for IInstallOptions3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions3",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AllowUpgradeToUnknownVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAllowUpgradeToUnknownVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions4, IInstallOptions4_Vtbl,
                0x50e8d7ad_c7c6_5e78_a794_83f5ef04f1b9
            );
            impl windows_core::RuntimeType for IInstallOptions4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions4",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Force: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetForce: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions5, IInstallOptions5_Vtbl,
                0xcda16279_1ebe_58e4_b976_6cc94d891dd9
            );
            impl windows_core::RuntimeType for IInstallOptions5 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions5",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions5_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AdditionalInstallerArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAdditionalInstallerArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub BypassIsStoreClientBlockedPolicyCheck: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetBypassIsStoreClientBlockedPolicyCheck: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions6, IInstallOptions6_Vtbl,
                0x0ccf6bed_ebc5_5504_97f5_7a757398fbfe
            );
            impl windows_core::RuntimeType for IInstallOptions6 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions6",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions6_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SkipDependencies: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetSkipDependencies: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub InstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallerType,
                ) -> windows_core::HRESULT,
                pub SetInstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageInstallerType,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallOptions7, IInstallOptions7_Vtbl,
                0xde8e9f42_99b0_5676_ac28_df475f77ddb2
            );
            impl windows_core::RuntimeType for IInstallOptions7 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallOptions7",
                );
            }
            #[repr(C)]
            pub struct IInstallOptions7_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallResult, IInstallResult_Vtbl,
                0x0d00ed2d_448b_58c4_8e12_5bb808845fa5
            );
            impl windows_core::RuntimeType for IInstallResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallResult",
                );
            }
            #[repr(C)]
            pub struct IInstallResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub RebootRequired: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut InstallResultStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstallResult2, IInstallResult2_Vtbl,
                0xa3e71b86_d480_5079_981a_60733e445adf
            );
            impl windows_core::RuntimeType for IInstallResult2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstallResult2",
                );
            }
            #[repr(C)]
            pub struct IInstallResult2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InstallerErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IInstalledStatus, IInstalledStatus_Vtbl,
                0x8cefd28c_1a3d_530e_9136_f08be7bf7d44
            );
            impl windows_core::RuntimeType for IInstalledStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IInstalledStatus",
                );
            }
            #[repr(C)]
            pub struct IInstalledStatus_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Type: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut InstalledStatusType,
                ) -> windows_core::HRESULT,
                pub Path: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IMatchResult, IMatchResult_Vtbl, 0x5500b7ae_25eb_5cd4_8fbb_9d1cb23b5696
            );
            impl windows_core::RuntimeType for IMatchResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IMatchResult",
                );
            }
            #[repr(C)]
            pub struct IMatchResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CatalogPackage: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub MatchCriteria: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IMicrosoftEntraIdAuthenticationInfo,
                IMicrosoftEntraIdAuthenticationInfo_Vtbl,
                0x516f86e9_bdae_5f81_8584_17cadc7b13cf
            );
            impl windows_core::RuntimeType for IMicrosoftEntraIdAuthenticationInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IMicrosoftEntraIdAuthenticationInfo",
                );
            }
            #[repr(C)]
            pub struct IMicrosoftEntraIdAuthenticationInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Resource: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Scope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageAgreement, IPackageAgreement_Vtbl,
                0x077c0159_5516_5920_9e8e_f15704224c42
            );
            impl windows_core::RuntimeType for IPackageAgreement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageAgreement",
                );
            }
            #[repr(C)]
            pub struct IPackageAgreement_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Label: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Text: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Url: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalog, IPackageCatalog_Vtbl,
                0x036a0492_9532_54e2_84a0_b4418e4403b4
            );
            impl windows_core::RuntimeType for IPackageCatalog {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalog",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalog_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsComposite: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Info: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub FindPackagesAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub FindPackages: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogConnectionValidationEventArgs,
                IPackageCatalogConnectionValidationEventArgs_Vtbl,
                0x2e1bdf2a_86a5_5f26_b7a1_a08a92ae1a97
            );
            impl windows_core::RuntimeType
            for IPackageCatalogConnectionValidationEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogConnectionValidationEventArgs",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogConnectionValidationEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogInfo, IPackageCatalogInfo_Vtbl,
                0xfaaaa46d_78f2_5567_b228_a7a6385eaed8
            );
            impl windows_core::RuntimeType for IPackageCatalogInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogInfo",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Id: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Name: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Type: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Argument: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub LastUpdateTime: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_time::DateTime,
                ) -> windows_core::HRESULT,
                pub Origin: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageCatalogOrigin,
                ) -> windows_core::HRESULT,
                pub TrustLevel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageCatalogTrustLevel,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogInfo2, IPackageCatalogInfo2_Vtbl,
                0xae6fb7db_40fd_52ec_bd01_86a06681afc3
            );
            impl windows_core::RuntimeType for IPackageCatalogInfo2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogInfo2",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogInfo2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Explicit: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogInfo3, IPackageCatalogInfo3_Vtbl,
                0x5a7fc7f4_c149_52ae_a7fc_e60b7bdde24f
            );
            impl windows_core::RuntimeType for IPackageCatalogInfo3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogInfo3",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogInfo3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Priority: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut i32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference, IPackageCatalogReference_Vtbl,
                0x84354608_3c95_55f0_8836_78ada3d2c203
            );
            impl windows_core::RuntimeType for IPackageCatalogReference {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsComposite: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Info: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ConnectAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Connect: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference2, IPackageCatalogReference2_Vtbl,
                0x1d1d76ef_3b7d_5383_ba5e_cdb0203c5e1c
            );
            impl windows_core::RuntimeType for IPackageCatalogReference2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference2",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AdditionalPackageCatalogArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAdditionalPackageCatalogArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference3, IPackageCatalogReference3_Vtbl,
                0x10fd6007_da3d_529c_a917_9138aae34c2e
            );
            impl windows_core::RuntimeType for IPackageCatalogReference3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference3",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SourceAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AcceptSourceAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAcceptSourceAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference4, IPackageCatalogReference4_Vtbl,
                0x12c6b72d_f7f6_501e_9cd2_dcd8b641a3aa
            );
            impl windows_core::RuntimeType for IPackageCatalogReference4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference4",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageCatalogBackgroundUpdateInterval: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_time::TimeSpan,
                ) -> windows_core::HRESULT,
                pub SetPackageCatalogBackgroundUpdateInterval: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows_time::TimeSpan,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference5, IPackageCatalogReference5_Vtbl,
                0x4c8df888_7a5d_5e03_8431_42a667bb6a7e
            );
            impl windows_core::RuntimeType for IPackageCatalogReference5 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference5",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference5_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InstalledPackageInformationOnly: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetInstalledPackageInformationOnly: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference6, IPackageCatalogReference6_Vtbl,
                0x2f0d3a5f_4537_5a31_b2dd_8b5d1785390b
            );
            impl windows_core::RuntimeType for IPackageCatalogReference6 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference6",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference6_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AuthenticationInfo: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference7, IPackageCatalogReference7_Vtbl,
                0x14675454_4a43_5c61_aa17_89b55148a9df
            );
            impl windows_core::RuntimeType for IPackageCatalogReference7 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference7",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference7_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub RefreshPackageCatalogAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageCatalogReference8, IPackageCatalogReference8_Vtbl,
                0x26be71ca_a8cc_534f_8e1a_1cbe452cd243
            );
            impl windows_core::RuntimeType for IPackageCatalogReference8 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageCatalogReference8",
                );
            }
            #[repr(C)]
            pub struct IPackageCatalogReference8_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ConnectionValidationHandler: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetConnectionValidationHandler: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub IsConnectionValidationHandlerEnabled: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageInstallerInfo, IPackageInstallerInfo_Vtbl,
                0x8dadb434_9875_5815_9c17_8223f70147ad
            );
            impl windows_core::RuntimeType for IPackageInstallerInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageInstallerInfo",
                );
            }
            #[repr(C)]
            pub struct IPackageInstallerInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallerType,
                ) -> windows_core::HRESULT,
                pub NestedInstallerType: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallerType,
                ) -> windows_core::HRESULT,
                pub Architecture: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut super::super::super::Windows::System::ProcessorArchitecture,
                ) -> windows_core::HRESULT,
                pub Scope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageInstallerScope,
                ) -> windows_core::HRESULT,
                pub Locale: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageInstallerInfo2, IPackageInstallerInfo2_Vtbl,
                0xab0fbfe6_4954_5212_a3c1_523b48a313e6
            );
            impl windows_core::RuntimeType for IPackageInstallerInfo2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageInstallerInfo2",
                );
            }
            #[repr(C)]
            pub struct IPackageInstallerInfo2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ElevationRequirement: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut ElevationRequirement,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageInstallerInfo3, IPackageInstallerInfo3_Vtbl,
                0xf4754c66_7bab_5859_89d9_9f814c07829c
            );
            impl windows_core::RuntimeType for IPackageInstallerInfo3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageInstallerInfo3",
                );
            }
            #[repr(C)]
            pub struct IPackageInstallerInfo3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationInfo: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageInstallerInstalledStatus, IPackageInstallerInstalledStatus_Vtbl,
                0x0f565afd_dc96_5dd7_8279_3d836f510067
            );
            impl windows_core::RuntimeType for IPackageInstallerInstalledStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageInstallerInstalledStatus",
                );
            }
            #[repr(C)]
            pub struct IPackageInstallerInstalledStatus_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InstallerInfo: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub InstallerInstalledStatus: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager, IPackageManager_Vtbl,
                0xb375e3b9_f2e0_5c93_87a7_b67497f7e593
            );
            impl windows_core::RuntimeType for IPackageManager {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager",
                );
            }
            #[repr(C)]
            pub struct IPackageManager_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetPackageCatalogs: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetPredefinedPackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PredefinedPackageCatalog,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetLocalPackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    LocalPackageCatalog,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetPackageCatalogByName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CreateCompositePackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub InstallPackageAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager2, IPackageManager2_Vtbl,
                0x97836b07_3c72_507e_a46d_2f9a56e1ab0a
            );
            impl windows_core::RuntimeType for IPackageManager2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager2",
                );
            }
            #[repr(C)]
            pub struct IPackageManager2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetInstallProgress: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager3, IPackageManager3_Vtbl,
                0x0e93b929_7276_5932_9b3d_b401a4210d97
            );
            impl windows_core::RuntimeType for IPackageManager3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager3",
                );
            }
            #[repr(C)]
            pub struct IPackageManager3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub UpgradePackageAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub UninstallPackageAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetUninstallProgress: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager4, IPackageManager4_Vtbl,
                0xb5d66836_40ab_510d_b93a_ee81a5a2e2b6
            );
            impl windows_core::RuntimeType for IPackageManager4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager4",
                );
            }
            #[repr(C)]
            pub struct IPackageManager4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub DownloadPackageAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetDownloadProgress: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager5, IPackageManager5_Vtbl,
                0x4cc8a261_a65e_5751_8892_cd9bccec50ba
            );
            impl windows_core::RuntimeType for IPackageManager5 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager5",
                );
            }
            #[repr(C)]
            pub struct IPackageManager5_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub RepairPackageAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager6, IPackageManager6_Vtbl,
                0xa8ca6a84_733c_5c5f_95cc_f54ac8a9c267
            );
            impl windows_core::RuntimeType for IPackageManager6 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager6",
                );
            }
            #[repr(C)]
            pub struct IPackageManager6_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AddPackageCatalogAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub RemovePackageCatalogAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager7, IPackageManager7_Vtbl,
                0x90eefb2d_222a_550a_8b52_31095ef7ce0a
            );
            impl windows_core::RuntimeType for IPackageManager7 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager7",
                );
            }
            #[repr(C)]
            pub struct IPackageManager7_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Version: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManager8, IPackageManager8_Vtbl,
                0xf9585ecc_2647_59ad_b2d9_5a3ba0c57597
            );
            impl windows_core::RuntimeType for IPackageManager8 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManager8",
                );
            }
            #[repr(C)]
            pub struct IPackageManager8_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub EditPackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManagerSettings, IPackageManagerSettings_Vtbl,
                0x6c94ce36_102f_5587_9e08_fdb5dbd6f7fe
            );
            impl windows_core::RuntimeType for IPackageManagerSettings {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManagerSettings",
                );
            }
            #[repr(C)]
            pub struct IPackageManagerSettings_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SetCallerIdentifier: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetStateIdentifier: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetUserSettings: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageManagerSettings2, IPackageManagerSettings2_Vtbl,
                0xea6b4211_bcf5_5572_9c32_0477a9d39b9d
            );
            impl windows_core::RuntimeType for IPackageManagerSettings2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageManagerSettings2",
                );
            }
            #[repr(C)]
            pub struct IPackageManagerSettings2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CanUnloadPreference: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetCanUnloadPreference: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub TerminationSignalMonitoring: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetTerminationSignalMonitoring: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageMatchFilter, IPackageMatchFilter_Vtbl,
                0xd981eca3_4de5_5ad7_967a_698c7d60fc3b
            );
            impl windows_core::RuntimeType for IPackageMatchFilter {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageMatchFilter",
                );
            }
            #[repr(C)]
            pub struct IPackageMatchFilter_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Option: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageFieldMatchOption,
                ) -> windows_core::HRESULT,
                pub SetOption: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageFieldMatchOption,
                ) -> windows_core::HRESULT,
                pub Field: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageMatchField,
                ) -> windows_core::HRESULT,
                pub SetField: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageMatchField,
                ) -> windows_core::HRESULT,
                pub Value: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageVersionId, IPackageVersionId_Vtbl,
                0x75a4b385_ee41_5607_a6c3_eb4dcb348093
            );
            impl windows_core::RuntimeType for IPackageVersionId {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageVersionId",
                );
            }
            #[repr(C)]
            pub struct IPackageVersionId_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageCatalogId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Version: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Channel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageVersionInfo, IPackageVersionInfo_Vtbl,
                0x13202ae0_8f11_58dd_a91c_917df75c6dc0
            );
            impl windows_core::RuntimeType for IPackageVersionInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageVersionInfo",
                );
            }
            #[repr(C)]
            pub struct IPackageVersionInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetMetadata: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageVersionMetadataField,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Id: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub DisplayName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Version: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Channel: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageFamilyNames: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ProductCodes: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageCatalog: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageVersionInfo2, IPackageVersionInfo2_Vtbl,
                0x69be5693_63b7_52d5_a66d_c3fc132c1c9b
            );
            impl windows_core::RuntimeType for IPackageVersionInfo2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageVersionInfo2",
                );
            }
            #[repr(C)]
            pub struct IPackageVersionInfo2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CompareToVersion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut CompareResult,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageVersionInfo3, IPackageVersionInfo3_Vtbl,
                0x9eeb5f0f_0c2d_5f4f_b097_e4e8ac4033f9
            );
            impl windows_core::RuntimeType for IPackageVersionInfo3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageVersionInfo3",
                );
            }
            #[repr(C)]
            pub struct IPackageVersionInfo3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub HasApplicableInstaller: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Publisher: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPackageVersionInfo4, IPackageVersionInfo4_Vtbl,
                0x80a48d20_594b_576a_b40e_4188ea373198
            );
            impl windows_core::RuntimeType for IPackageVersionInfo4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IPackageVersionInfo4",
                );
            }
            #[repr(C)]
            pub struct IPackageVersionInfo4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetCatalogPackageMetadata: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetCatalogPackageMetadata2: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetApplicableInstaller: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRefreshPackageCatalogResult, IRefreshPackageCatalogResult_Vtbl,
                0x525e4e5e_4bf0_58c4_ae48_f7c5803bcc9b
            );
            impl windows_core::RuntimeType for IRefreshPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRefreshPackageCatalogResult",
                );
            }
            #[repr(C)]
            pub struct IRefreshPackageCatalogResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut RefreshPackageCatalogStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRemovePackageCatalogOptions, IRemovePackageCatalogOptions_Vtbl,
                0xd2e0f22a_a5e5_5ea7_9a3b_ef74469bb6d1
            );
            impl windows_core::RuntimeType for IRemovePackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRemovePackageCatalogOptions",
                );
            }
            #[repr(C)]
            pub struct IRemovePackageCatalogOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Name: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PreserveData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetPreserveData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRemovePackageCatalogResult, IRemovePackageCatalogResult_Vtbl,
                0x89f57e97_b99d_5420_a9f5_a3a002668b3b
            );
            impl windows_core::RuntimeType for IRemovePackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRemovePackageCatalogResult",
                );
            }
            #[repr(C)]
            pub struct IRemovePackageCatalogResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut RemovePackageCatalogStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRepairOptions, IRepairOptions_Vtbl,
                0x263f0546_2d7e_53a0_b8d1_75b74817ff18
            );
            impl windows_core::RuntimeType for IRepairOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRepairOptions",
                );
            }
            #[repr(C)]
            pub struct IRepairOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageRepairScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageRepairScope,
                ) -> windows_core::HRESULT,
                pub SetPackageRepairScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageRepairScope,
                ) -> windows_core::HRESULT,
                pub PackageRepairMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageRepairMode,
                ) -> windows_core::HRESULT,
                pub SetPackageRepairMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageRepairMode,
                ) -> windows_core::HRESULT,
                pub AcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAcceptPackageAgreements: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetCorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub AllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetAllowHashMismatch: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub LogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetLogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Force: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetForce: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub BypassIsStoreClientBlockedPolicyCheck: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetBypassIsStoreClientBlockedPolicyCheck: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRepairOptions2, IRepairOptions2_Vtbl,
                0xfff8f413_91e7_5e83_877a_4efbf784f519
            );
            impl windows_core::RuntimeType for IRepairOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRepairOptions2",
                );
            }
            #[repr(C)]
            pub struct IRepairOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetAuthenticationArguments: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRepairResult, IRepairResult_Vtbl, 0x35feac12_9b49_5217_9fbd_31f159e5049e
            );
            impl windows_core::RuntimeType for IRepairResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IRepairResult",
                );
            }
            #[repr(C)]
            pub struct IRepairResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub RebootRequired: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut RepairResultStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
                pub RepairerErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ISourceAgreement, ISourceAgreement_Vtbl,
                0xae4fc30f_01eb_519e_afe6_e60947022e9d
            );
            impl windows_core::RuntimeType for ISourceAgreement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.ISourceAgreement",
                );
            }
            #[repr(C)]
            pub struct ISourceAgreement_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Label: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Text: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Url: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUninstallOptions, IUninstallOptions_Vtbl,
                0x3ebc67f0_8339_594b_8a42_f90b69d02bbe
            );
            impl windows_core::RuntimeType for IUninstallOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IUninstallOptions",
                );
            }
            #[repr(C)]
            pub struct IUninstallOptions_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub PackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetPackageVersionId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub PackageUninstallMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageUninstallMode,
                ) -> windows_core::HRESULT,
                pub SetPackageUninstallMode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageUninstallMode,
                ) -> windows_core::HRESULT,
                pub LogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetLogOutputPath: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetCorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUninstallOptions2, IUninstallOptions2_Vtbl,
                0x7efaca1c_c3f5_53c8_a671_0c3451ed8655
            );
            impl windows_core::RuntimeType for IUninstallOptions2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IUninstallOptions2",
                );
            }
            #[repr(C)]
            pub struct IUninstallOptions2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Force: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub SetForce: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub PackageUninstallScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut PackageUninstallScope,
                ) -> windows_core::HRESULT,
                pub SetPackageUninstallScope: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    PackageUninstallScope,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUninstallResult, IUninstallResult_Vtbl,
                0xfc22eb55_e8ba_5819_9ffa_b0c0f8425cc1
            );
            impl windows_core::RuntimeType for IUninstallResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IUninstallResult",
                );
            }
            #[repr(C)]
            pub struct IUninstallResult_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CorrelationData: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub RebootRequired: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub Status: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut UninstallResultStatus,
                ) -> windows_core::HRESULT,
                pub ExtendedErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut windows_core::HRESULT,
                ) -> windows_core::HRESULT,
                pub UninstallerErrorCode: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Icon(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                Icon, windows_core::IUnknown, windows_core::IInspectable
            );
            impl Icon {
                pub fn Url(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Url)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn FileType(&self) -> windows_core::Result<IconFileType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .FileType)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Resolution(&self) -> windows_core::Result<IconResolution> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Resolution)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Theme(&self) -> windows_core::Result<IconTheme> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Theme)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Sha256(&self) -> windows_core::Result<windows_core::Array<u8>> {
                    unsafe {
                        let mut result__ = core::mem::MaybeUninit::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Sha256)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<
                                    u8,
                                >::set_abi_len(core::mem::transmute(&mut result__)),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                    }
                }
            }
            impl windows_core::RuntimeType for Icon {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IIcon,
                >();
            }
            unsafe impl windows_core::Interface for Icon {
                type Vtable = <IIcon as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IIcon as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Icon {
                const NAME: &'static str = "Microsoft.Management.Deployment.Icon";
            }
            unsafe impl Send for Icon {}
            unsafe impl Sync for Icon {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct IconFileType(pub i32);
            impl IconFileType {
                pub const Unknown: Self = Self(0);
                pub const Jpeg: Self = Self(1);
                pub const Png: Self = Self(2);
                pub const Ico: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for IconFileType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for IconFileType {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.IconFileType;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IconFileType",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct IconResolution(pub i32);
            impl IconResolution {
                pub const Custom: Self = Self(0);
                pub const Square16: Self = Self(1);
                pub const Square20: Self = Self(2);
                pub const Square24: Self = Self(3);
                pub const Square30: Self = Self(4);
                pub const Square32: Self = Self(5);
                pub const Square36: Self = Self(6);
                pub const Square40: Self = Self(7);
                pub const Square48: Self = Self(8);
                pub const Square60: Self = Self(9);
                pub const Square64: Self = Self(10);
                pub const Square72: Self = Self(11);
                pub const Square80: Self = Self(12);
                pub const Square96: Self = Self(13);
                pub const Square256: Self = Self(14);
            }
            impl windows_core::imp::TypeKind for IconResolution {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for IconResolution {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.IconResolution;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IconResolution",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct IconTheme(pub i32);
            impl IconTheme {
                pub const Unknown: Self = Self(0);
                pub const Default: Self = Self(1);
                pub const Light: Self = Self(2);
                pub const Dark: Self = Self(3);
                pub const HighContrast: Self = Self(4);
            }
            impl windows_core::imp::TypeKind for IconTheme {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for IconTheme {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.IconTheme;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.IconTheme",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct InstallOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                InstallOptions, windows_core::IUnknown, windows_core::IInspectable
            );
            impl InstallOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        InstallOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn PackageVersionId(
                    &self,
                ) -> windows_core::Result<PackageVersionId> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetPackageVersionId<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<PackageVersionId>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn PreferredInstallLocation(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PreferredInstallLocation)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetPreferredInstallLocation(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPreferredInstallLocation)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn PackageInstallScope(
                    &self,
                ) -> windows_core::Result<PackageInstallScope> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageInstallScope)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageInstallScope(
                    &self,
                    value: PackageInstallScope,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageInstallScope)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn PackageInstallMode(
                    &self,
                ) -> windows_core::Result<PackageInstallMode> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageInstallMode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageInstallMode(
                    &self,
                    value: PackageInstallMode,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageInstallMode)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn LogOutputPath(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .LogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetLogOutputPath(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetLogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AllowHashMismatch(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAllowHashMismatch(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn ReplacementInstallerArguments(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ReplacementInstallerArguments)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetReplacementInstallerArguments(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetReplacementInstallerArguments)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetCorrelationData(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCorrelationData)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AdditionalPackageCatalogArguments(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AdditionalPackageCatalogArguments)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetAdditionalPackageCatalogArguments(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAdditionalPackageCatalogArguments)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AllowedArchitectures(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVector<
                        super::super::super::Windows::System::ProcessorArchitecture,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<IInstallOptions2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AllowedArchitectures)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn AllowUpgradeToUnknownVersion(
                    &self,
                ) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IInstallOptions3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AllowUpgradeToUnknownVersion)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAllowUpgradeToUnknownVersion(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions3>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAllowUpgradeToUnknownVersion)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn Force(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IInstallOptions4>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Force)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetForce(&self, value: bool) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions4>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetForce)(windows_core::Interface::as_raw(this), value)
                            .ok()
                    }
                }
                pub fn AdditionalInstallerArguments(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AdditionalInstallerArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetAdditionalInstallerArguments(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAdditionalInstallerArguments)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AcceptPackageAgreements(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AcceptPackageAgreements)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAcceptPackageAgreements(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAcceptPackageAgreements)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn BypassIsStoreClientBlockedPolicyCheck(
                    &self,
                ) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .BypassIsStoreClientBlockedPolicyCheck)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetBypassIsStoreClientBlockedPolicyCheck(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions5>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetBypassIsStoreClientBlockedPolicyCheck)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn SkipDependencies(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IInstallOptions6>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .SkipDependencies)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetSkipDependencies(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions6>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetSkipDependencies)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn InstallerType(
                    &self,
                ) -> windows_core::Result<PackageInstallerType> {
                    let this = &windows_core::Interface::cast::<IInstallOptions6>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .InstallerType)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetInstallerType(
                    &self,
                    value: PackageInstallerType,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IInstallOptions6>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetInstallerType)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationArguments(
                    &self,
                ) -> windows_core::Result<AuthenticationArguments> {
                    let this = &windows_core::Interface::cast::<IInstallOptions7>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetAuthenticationArguments<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<AuthenticationArguments>,
                {
                    let this = &windows_core::Interface::cast::<IInstallOptions7>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for InstallOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IInstallOptions,
                >();
            }
            unsafe impl windows_core::Interface for InstallOptions {
                type Vtable = <IInstallOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IInstallOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for InstallOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.InstallOptions";
            }
            unsafe impl Send for InstallOptions {}
            unsafe impl Sync for InstallOptions {}
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct InstallProgress {
                pub State: PackageInstallProgressState,
                pub BytesDownloaded: u64,
                pub BytesRequired: u64,
                pub DownloadProgress: f64,
                pub InstallationProgress: f64,
            }
            impl windows_core::imp::TypeKind for InstallProgress {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for InstallProgress {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Microsoft.Management.Deployment.InstallProgress;enum(Microsoft.Management.Deployment.PackageInstallProgressState;i4);u8;u8;f8;f8)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.InstallProgress",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct InstallResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                InstallResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl InstallResult {
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn RebootRequired(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .RebootRequired)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Status(&self) -> windows_core::Result<InstallResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn InstallerErrorCode(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<IInstallResult2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .InstallerErrorCode)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for InstallResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IInstallResult,
                >();
            }
            unsafe impl windows_core::Interface for InstallResult {
                type Vtable = <IInstallResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IInstallResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for InstallResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.InstallResult";
            }
            unsafe impl Send for InstallResult {}
            unsafe impl Sync for InstallResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct InstallResultStatus(pub i32);
            impl InstallResultStatus {
                pub const Ok: Self = Self(0);
                pub const BlockedByPolicy: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const InvalidOptions: Self = Self(4);
                pub const DownloadError: Self = Self(5);
                pub const InstallError: Self = Self(6);
                pub const ManifestError: Self = Self(7);
                pub const NoApplicableInstallers: Self = Self(8);
                pub const NoApplicableUpgrade: Self = Self(9);
                pub const PackageAgreementsNotAccepted: Self = Self(10);
            }
            impl windows_core::imp::TypeKind for InstallResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for InstallResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.InstallResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.InstallResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct InstalledStatus(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                InstalledStatus, windows_core::IUnknown, windows_core::IInspectable
            );
            impl InstalledStatus {
                pub fn Type(&self) -> windows_core::Result<InstalledStatusType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Type)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Path)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Status(&self) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for InstalledStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IInstalledStatus,
                >();
            }
            unsafe impl windows_core::Interface for InstalledStatus {
                type Vtable = <IInstalledStatus as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IInstalledStatus as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for InstalledStatus {
                const NAME: &'static str = "Microsoft.Management.Deployment.InstalledStatus";
            }
            unsafe impl Send for InstalledStatus {}
            unsafe impl Sync for InstalledStatus {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct InstalledStatusType(pub u32);
            impl InstalledStatusType {
                pub const None: Self = Self(0);
                pub const AppsAndFeaturesEntry: Self = Self(1);
                pub const AppsAndFeaturesEntryInstallLocation: Self = Self(2);
                pub const AppsAndFeaturesEntryInstallLocationFile: Self = Self(4);
                pub const DefaultInstallLocation: Self = Self(8);
                pub const DefaultInstallLocationFile: Self = Self(16);
                pub const AllAppsAndFeaturesEntryChecks: Self = Self(7);
                pub const AllDefaultInstallLocationChecks: Self = Self(24);
                pub const AllChecks: Self = Self(31);
            }
            impl windows_core::imp::TypeKind for InstalledStatusType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for InstalledStatusType {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.InstalledStatusType;u4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.InstalledStatusType",
                );
            }
            impl InstalledStatusType {
                pub const fn contains(&self, other: Self) -> bool {
                    self.0 & other.0 == other.0
                }
            }
            impl core::ops::BitOr for InstalledStatusType {
                type Output = Self;
                fn bitor(self, other: Self) -> Self {
                    Self(self.0 | other.0)
                }
            }
            impl core::ops::BitAnd for InstalledStatusType {
                type Output = Self;
                fn bitand(self, other: Self) -> Self {
                    Self(self.0 & other.0)
                }
            }
            impl core::ops::BitOrAssign for InstalledStatusType {
                fn bitor_assign(&mut self, other: Self) {
                    self.0.bitor_assign(other.0);
                }
            }
            impl core::ops::BitAndAssign for InstalledStatusType {
                fn bitand_assign(&mut self, other: Self) {
                    self.0.bitand_assign(other.0);
                }
            }
            impl core::ops::Not for InstalledStatusType {
                type Output = Self;
                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct LocalPackageCatalog(pub i32);
            impl LocalPackageCatalog {
                pub const InstalledPackages: Self = Self(0);
                pub const InstallingPackages: Self = Self(1);
            }
            impl windows_core::imp::TypeKind for LocalPackageCatalog {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for LocalPackageCatalog {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.LocalPackageCatalog;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.LocalPackageCatalog",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct MatchResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                MatchResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl MatchResult {
                pub fn CatalogPackage(&self) -> windows_core::Result<CatalogPackage> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CatalogPackage)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn MatchCriteria(&self) -> windows_core::Result<PackageMatchFilter> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .MatchCriteria)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for MatchResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IMatchResult,
                >();
            }
            unsafe impl windows_core::Interface for MatchResult {
                type Vtable = <IMatchResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IMatchResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for MatchResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.MatchResult";
            }
            unsafe impl Send for MatchResult {}
            unsafe impl Sync for MatchResult {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct MicrosoftEntraIdAuthenticationInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                MicrosoftEntraIdAuthenticationInfo, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl MicrosoftEntraIdAuthenticationInfo {
                pub fn Resource(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Resource)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Scope(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Scope)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for MicrosoftEntraIdAuthenticationInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IMicrosoftEntraIdAuthenticationInfo,
                >();
            }
            unsafe impl windows_core::Interface for MicrosoftEntraIdAuthenticationInfo {
                type Vtable = <IMicrosoftEntraIdAuthenticationInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IMicrosoftEntraIdAuthenticationInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for MicrosoftEntraIdAuthenticationInfo {
                const NAME: &'static str = "Microsoft.Management.Deployment.MicrosoftEntraIdAuthenticationInfo";
            }
            unsafe impl Send for MicrosoftEntraIdAuthenticationInfo {}
            unsafe impl Sync for MicrosoftEntraIdAuthenticationInfo {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageAgreement(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageAgreement, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageAgreement {
                pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Label)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Text)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Url(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Url)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageAgreement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageAgreement,
                >();
            }
            unsafe impl windows_core::Interface for PackageAgreement {
                type Vtable = <IPackageAgreement as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageAgreement as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageAgreement {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageAgreement";
            }
            unsafe impl Send for PackageAgreement {}
            unsafe impl Sync for PackageAgreement {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageCatalog(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageCatalog, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageCatalog {
                pub fn IsComposite(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .IsComposite)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Info(&self) -> windows_core::Result<PackageCatalogInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Info)(windows_core::Interface::as_raw(self), &mut result__)
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn FindPackagesAsync<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperation<FindPackagesResult>,
                >
                where
                    P0: windows_core::Param<FindPackagesOptions>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .FindPackagesAsync)(
                                windows_core::Interface::as_raw(self),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn FindPackages<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<FindPackagesResult>
                where
                    P0: windows_core::Param<FindPackagesOptions>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .FindPackages)(
                                windows_core::Interface::as_raw(self),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageCatalog {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageCatalog,
                >();
            }
            unsafe impl windows_core::Interface for PackageCatalog {
                type Vtable = <IPackageCatalog as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageCatalog as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageCatalog {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageCatalog";
            }
            unsafe impl Send for PackageCatalog {}
            unsafe impl Sync for PackageCatalog {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageCatalogConnectionValidationEventArgs(
                windows_core::IUnknown,
            );
            windows_core::imp::interface_hierarchy!(
                PackageCatalogConnectionValidationEventArgs, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType
            for PackageCatalogConnectionValidationEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageCatalogConnectionValidationEventArgs,
                >();
            }
            unsafe impl windows_core::Interface
            for PackageCatalogConnectionValidationEventArgs {
                type Vtable = <IPackageCatalogConnectionValidationEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageCatalogConnectionValidationEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName
            for PackageCatalogConnectionValidationEventArgs {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageCatalogConnectionValidationEventArgs";
            }
            unsafe impl Send for PackageCatalogConnectionValidationEventArgs {}
            unsafe impl Sync for PackageCatalogConnectionValidationEventArgs {}
            windows_core::imp::define_interface!(
                PackageCatalogConnectionValidationHandler,
                PackageCatalogConnectionValidationHandler_Vtbl,
                0x1c492bda_41eb_50e1_b00e_e536df58ae9e
            );
            impl windows_core::RuntimeType
            for PackageCatalogConnectionValidationHandler {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<
                    Self,
                >();
            }
            impl PackageCatalogConnectionValidationHandler {
                pub fn new<
                    F: Fn(
                            windows_core::Ref<
                                PackageCatalogConnectionValidationEventArgs,
                            >,
                        ) -> windows_core::Result<
                                PackageCatalogConnectionValidationResult,
                            > + Send + 'static,
                >(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<
                        Self,
                        F,
                    >::new(
                        &PackageCatalogConnectionValidationHandlerBox::<F>::VTABLE,
                        invoke,
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                pub fn Invoke<P0>(
                    &self,
                    args: P0,
                ) -> windows_core::Result<PackageCatalogConnectionValidationResult>
                where
                    P0: windows_core::Param<PackageCatalogConnectionValidationEventArgs>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Invoke)(
                                windows_core::Interface::as_raw(self),
                                args.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            #[repr(C)]
            pub struct PackageCatalogConnectionValidationHandler_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(
                    this: *mut core::ffi::c_void,
                    args: *mut core::ffi::c_void,
                    result__: *mut PackageCatalogConnectionValidationResult,
                ) -> windows_core::HRESULT,
            }
            struct PackageCatalogConnectionValidationHandlerBox<
                F: Fn(
                        windows_core::Ref<PackageCatalogConnectionValidationEventArgs>,
                    ) -> windows_core::Result<PackageCatalogConnectionValidationResult>
                    + Send + 'static,
            >(
                core::marker::PhantomData<(fn() -> F,)>,
            );
            impl<
                F: Fn(
                        windows_core::Ref<PackageCatalogConnectionValidationEventArgs>,
                    ) -> windows_core::Result<PackageCatalogConnectionValidationResult>
                    + Send + 'static,
            > PackageCatalogConnectionValidationHandlerBox<F> {
                const VTABLE: PackageCatalogConnectionValidationHandler_Vtbl = PackageCatalogConnectionValidationHandler_Vtbl {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: windows_core::imp::DelegateBox::<
                            PackageCatalogConnectionValidationHandler,
                            F,
                        >::QueryInterface,
                        AddRef: windows_core::imp::DelegateBox::<
                            PackageCatalogConnectionValidationHandler,
                            F,
                        >::AddRef,
                        Release: windows_core::imp::DelegateBox::<
                            PackageCatalogConnectionValidationHandler,
                            F,
                        >::Release,
                    },
                    Invoke: Self::Invoke,
                };
                unsafe extern "system" fn Invoke(
                    this: *mut core::ffi::c_void,
                    args: *mut core::ffi::c_void,
                    result__: *mut PackageCatalogConnectionValidationResult,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void
                            as *mut windows_core::imp::DelegateBox<
                                PackageCatalogConnectionValidationHandler,
                                F,
                            >);
                        match (this.invoke)(core::mem::transmute_copy(&args)) {
                            Ok(ok__) => {
                                result__.write(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageCatalogConnectionValidationResult(pub i32);
            impl PackageCatalogConnectionValidationResult {
                pub const Ok: Self = Self(0);
                pub const CertificateRejected: Self = Self(1);
            }
            impl windows_core::imp::TypeKind
            for PackageCatalogConnectionValidationResult {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageCatalogConnectionValidationResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageCatalogConnectionValidationResult;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageCatalogConnectionValidationResult",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageCatalogInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageCatalogInfo, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageCatalogInfo {
                pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Id)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Name)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Type(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Type)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Argument(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Argument)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn LastUpdateTime(
                    &self,
                ) -> windows_core::Result<windows_time::DateTime> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .LastUpdateTime)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Origin(&self) -> windows_core::Result<PackageCatalogOrigin> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Origin)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn TrustLevel(
                    &self,
                ) -> windows_core::Result<PackageCatalogTrustLevel> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .TrustLevel)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Explicit(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogInfo2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Explicit)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Priority(&self) -> windows_core::Result<i32> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogInfo3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Priority)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for PackageCatalogInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageCatalogInfo,
                >();
            }
            unsafe impl windows_core::Interface for PackageCatalogInfo {
                type Vtable = <IPackageCatalogInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageCatalogInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageCatalogInfo {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageCatalogInfo";
            }
            unsafe impl Send for PackageCatalogInfo {}
            unsafe impl Sync for PackageCatalogInfo {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageCatalogOrigin(pub i32);
            impl PackageCatalogOrigin {
                pub const Predefined: Self = Self(0);
                pub const User: Self = Self(1);
            }
            impl windows_core::imp::TypeKind for PackageCatalogOrigin {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageCatalogOrigin {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageCatalogOrigin;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageCatalogOrigin",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageCatalogReference(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageCatalogReference, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl PackageCatalogReference {
                pub fn IsComposite(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .IsComposite)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Info(&self) -> windows_core::Result<PackageCatalogInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Info)(windows_core::Interface::as_raw(self), &mut result__)
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ConnectAsync(
                    &self,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperation<ConnectResult>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ConnectAsync)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Connect(&self) -> windows_core::Result<ConnectResult> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Connect)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn AdditionalPackageCatalogArguments(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AdditionalPackageCatalogArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetAdditionalPackageCatalogArguments(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAdditionalPackageCatalogArguments)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn SourceAgreements(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<SourceAgreement>,
                > {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .SourceAgreements)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn AcceptSourceAgreements(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AcceptSourceAgreements)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAcceptSourceAgreements(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference3,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAcceptSourceAgreements)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn PackageCatalogBackgroundUpdateInterval(
                    &self,
                ) -> windows_core::Result<windows_time::TimeSpan> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference4,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .PackageCatalogBackgroundUpdateInterval)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageCatalogBackgroundUpdateInterval(
                    &self,
                    value: windows_time::TimeSpan,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference4,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetPackageCatalogBackgroundUpdateInterval)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn InstalledPackageInformationOnly(
                    &self,
                ) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference5,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .InstalledPackageInformationOnly)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetInstalledPackageInformationOnly(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference5,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetInstalledPackageInformationOnly)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationArguments(
                    &self,
                ) -> windows_core::Result<AuthenticationArguments> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference6,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetAuthenticationArguments<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<AuthenticationArguments>,
                {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference6,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationInfo(
                    &self,
                ) -> windows_core::Result<AuthenticationInfo> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference6,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationInfo)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn RefreshPackageCatalogAsync(
                    &self,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        RefreshPackageCatalogResult,
                        f64,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference7,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .RefreshPackageCatalogAsync)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ConnectionValidationHandler(
                    &self,
                ) -> windows_core::Result<PackageCatalogConnectionValidationHandler> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference8,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .ConnectionValidationHandler)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetConnectionValidationHandler<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<PackageCatalogConnectionValidationHandler>,
                {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference8,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetConnectionValidationHandler)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn IsConnectionValidationHandlerEnabled(
                    &self,
                ) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageCatalogReference8,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .IsConnectionValidationHandlerEnabled)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for PackageCatalogReference {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageCatalogReference,
                >();
            }
            unsafe impl windows_core::Interface for PackageCatalogReference {
                type Vtable = <IPackageCatalogReference as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageCatalogReference as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageCatalogReference {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageCatalogReference";
            }
            unsafe impl Send for PackageCatalogReference {}
            unsafe impl Sync for PackageCatalogReference {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageCatalogTrustLevel(pub i32);
            impl PackageCatalogTrustLevel {
                pub const None: Self = Self(0);
                pub const Trusted: Self = Self(1);
            }
            impl windows_core::imp::TypeKind for PackageCatalogTrustLevel {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageCatalogTrustLevel {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageCatalogTrustLevel;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageCatalogTrustLevel",
                );
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct PackageDownloadProgress {
                pub State: PackageDownloadProgressState,
                pub BytesDownloaded: u64,
                pub BytesRequired: u64,
                pub DownloadProgress: f64,
            }
            impl windows_core::imp::TypeKind for PackageDownloadProgress {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageDownloadProgress {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Microsoft.Management.Deployment.PackageDownloadProgress;enum(Microsoft.Management.Deployment.PackageDownloadProgressState;i4);u8;u8;f8)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageDownloadProgress",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageDownloadProgressState(pub i32);
            impl PackageDownloadProgressState {
                pub const Queued: Self = Self(0);
                pub const Downloading: Self = Self(1);
                pub const Finished: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageDownloadProgressState {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageDownloadProgressState {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageDownloadProgressState;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageDownloadProgressState",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageFieldMatchOption(pub i32);
            impl PackageFieldMatchOption {
                pub const Equals: Self = Self(0);
                pub const EqualsCaseInsensitive: Self = Self(1);
                pub const StartsWithCaseInsensitive: Self = Self(2);
                pub const ContainsCaseInsensitive: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for PackageFieldMatchOption {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageFieldMatchOption {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageFieldMatchOption;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageFieldMatchOption",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageInstallMode(pub i32);
            impl PackageInstallMode {
                pub const Default: Self = Self(0);
                pub const Silent: Self = Self(1);
                pub const Interactive: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageInstallMode {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageInstallMode {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageInstallMode;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageInstallMode",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageInstallProgressState(pub i32);
            impl PackageInstallProgressState {
                pub const Queued: Self = Self(0);
                pub const Downloading: Self = Self(1);
                pub const Installing: Self = Self(2);
                pub const PostInstall: Self = Self(3);
                pub const Finished: Self = Self(4);
            }
            impl windows_core::imp::TypeKind for PackageInstallProgressState {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageInstallProgressState {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageInstallProgressState;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageInstallProgressState",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageInstallScope(pub i32);
            impl PackageInstallScope {
                pub const Any: Self = Self(0);
                pub const User: Self = Self(1);
                pub const System: Self = Self(2);
                pub const UserOrUnknown: Self = Self(3);
                pub const SystemOrUnknown: Self = Self(4);
            }
            impl windows_core::imp::TypeKind for PackageInstallScope {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageInstallScope {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageInstallScope;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageInstallScope",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageInstallerInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageInstallerInfo, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageInstallerInfo {
                pub fn InstallerType(
                    &self,
                ) -> windows_core::Result<PackageInstallerType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallerType)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn NestedInstallerType(
                    &self,
                ) -> windows_core::Result<PackageInstallerType> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .NestedInstallerType)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Architecture(
                    &self,
                ) -> windows_core::Result<
                    super::super::super::Windows::System::ProcessorArchitecture,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Architecture)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Scope(&self) -> windows_core::Result<PackageInstallerScope> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Scope)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Locale(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Locale)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn ElevationRequirement(
                    &self,
                ) -> windows_core::Result<ElevationRequirement> {
                    let this = &windows_core::Interface::cast::<
                        IPackageInstallerInfo2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .ElevationRequirement)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn AuthenticationInfo(
                    &self,
                ) -> windows_core::Result<AuthenticationInfo> {
                    let this = &windows_core::Interface::cast::<
                        IPackageInstallerInfo3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationInfo)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageInstallerInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageInstallerInfo,
                >();
            }
            unsafe impl windows_core::Interface for PackageInstallerInfo {
                type Vtable = <IPackageInstallerInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageInstallerInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageInstallerInfo {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageInstallerInfo";
            }
            unsafe impl Send for PackageInstallerInfo {}
            unsafe impl Sync for PackageInstallerInfo {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageInstallerInstalledStatus(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageInstallerInstalledStatus, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl PackageInstallerInstalledStatus {
                pub fn InstallerInfo(
                    &self,
                ) -> windows_core::Result<PackageInstallerInfo> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallerInfo)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn InstallerInstalledStatus(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<InstalledStatus>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallerInstalledStatus)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageInstallerInstalledStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageInstallerInstalledStatus,
                >();
            }
            unsafe impl windows_core::Interface for PackageInstallerInstalledStatus {
                type Vtable = <IPackageInstallerInstalledStatus as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageInstallerInstalledStatus as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageInstallerInstalledStatus {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageInstallerInstalledStatus";
            }
            unsafe impl Send for PackageInstallerInstalledStatus {}
            unsafe impl Sync for PackageInstallerInstalledStatus {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageInstallerScope(pub i32);
            impl PackageInstallerScope {
                pub const Unknown: Self = Self(0);
                pub const User: Self = Self(1);
                pub const System: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageInstallerScope {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageInstallerScope {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageInstallerScope;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageInstallerScope",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageInstallerType(pub i32);
            impl PackageInstallerType {
                pub const Unknown: Self = Self(0);
                pub const Inno: Self = Self(1);
                pub const Wix: Self = Self(2);
                pub const Msi: Self = Self(3);
                pub const Nullsoft: Self = Self(4);
                pub const Zip: Self = Self(5);
                pub const Msix: Self = Self(6);
                pub const Exe: Self = Self(7);
                pub const Burn: Self = Self(8);
                pub const MSStore: Self = Self(9);
                pub const Portable: Self = Self(10);
                pub const Font: Self = Self(11);
            }
            impl windows_core::imp::TypeKind for PackageInstallerType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageInstallerType {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageInstallerType;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageInstallerType",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageManager(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageManager, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageManager {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        PackageManager,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn GetPackageCatalogs(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<PackageCatalogReference>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetPackageCatalogs)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetPredefinedPackageCatalog(
                    &self,
                    predefinedpackagecatalog: PredefinedPackageCatalog,
                ) -> windows_core::Result<PackageCatalogReference> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetPredefinedPackageCatalog)(
                                windows_core::Interface::as_raw(self),
                                predefinedpackagecatalog,
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetLocalPackageCatalog(
                    &self,
                    localpackagecatalog: LocalPackageCatalog,
                ) -> windows_core::Result<PackageCatalogReference> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetLocalPackageCatalog)(
                                windows_core::Interface::as_raw(self),
                                localpackagecatalog,
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetPackageCatalogByName(
                    &self,
                    catalogname: &windows_core::HSTRING,
                ) -> windows_core::Result<PackageCatalogReference> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetPackageCatalogByName)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(catalogname),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CreateCompositePackageCatalog<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<PackageCatalogReference>
                where
                    P0: windows_core::Param<CreateCompositePackageCatalogOptions>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CreateCompositePackageCatalog)(
                                windows_core::Interface::as_raw(self),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn InstallPackageAsync<P0, P1>(
                    &self,
                    package: P0,
                    options: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        InstallResult,
                        InstallProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<InstallOptions>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .InstallPackageAsync)(
                                windows_core::Interface::as_raw(self),
                                package.param().abi(),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetInstallProgress<P0, P1>(
                    &self,
                    package: P0,
                    cataloginfo: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        InstallResult,
                        InstallProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<PackageCatalogInfo>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetInstallProgress)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                cataloginfo.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn UpgradePackageAsync<P0, P1>(
                    &self,
                    package: P0,
                    options: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        InstallResult,
                        InstallProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<InstallOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .UpgradePackageAsync)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn UninstallPackageAsync<P0, P1>(
                    &self,
                    package: P0,
                    options: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        UninstallResult,
                        UninstallProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<UninstallOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .UninstallPackageAsync)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetUninstallProgress<P0, P1>(
                    &self,
                    package: P0,
                    cataloginfo: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        UninstallResult,
                        UninstallProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<PackageCatalogInfo>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetUninstallProgress)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                cataloginfo.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn DownloadPackageAsync<P0, P1>(
                    &self,
                    package: P0,
                    options: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        DownloadResult,
                        PackageDownloadProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<DownloadOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager4>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .DownloadPackageAsync)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetDownloadProgress<P0, P1>(
                    &self,
                    package: P0,
                    cataloginfo: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        DownloadResult,
                        PackageDownloadProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<PackageCatalogInfo>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager4>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetDownloadProgress)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                cataloginfo.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn RepairPackageAsync<P0, P1>(
                    &self,
                    package: P0,
                    options: P1,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        RepairResult,
                        RepairProgress,
                    >,
                >
                where
                    P0: windows_core::Param<CatalogPackage>,
                    P1: windows_core::Param<RepairOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager5>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .RepairPackageAsync)(
                                windows_core::Interface::as_raw(this),
                                package.param().abi(),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn AddPackageCatalogAsync<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        AddPackageCatalogResult,
                        f64,
                    >,
                >
                where
                    P0: windows_core::Param<AddPackageCatalogOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AddPackageCatalogAsync)(
                                windows_core::Interface::as_raw(this),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn RemovePackageCatalogAsync<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<
                    windows_future::IAsyncOperationWithProgress<
                        RemovePackageCatalogResult,
                        f64,
                    >,
                >
                where
                    P0: windows_core::Param<RemovePackageCatalogOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .RemovePackageCatalogAsync)(
                                windows_core::Interface::as_raw(this),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<IPackageManager7>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Version)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn EditPackageCatalog<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<EditPackageCatalogResult>
                where
                    P0: windows_core::Param<EditPackageCatalogOptions>,
                {
                    let this = &windows_core::Interface::cast::<IPackageManager8>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .EditPackageCatalog)(
                                windows_core::Interface::as_raw(this),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageManager {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageManager,
                >();
            }
            unsafe impl windows_core::Interface for PackageManager {
                type Vtable = <IPackageManager as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageManager as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageManager {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageManager";
            }
            unsafe impl Send for PackageManager {}
            unsafe impl Sync for PackageManager {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageManagerSettings(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageManagerSettings, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl PackageManagerSettings {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        PackageManagerSettings,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn SetCallerIdentifier(
                    &self,
                    calleridentifier: &windows_core::HSTRING,
                ) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .SetCallerIdentifier)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(calleridentifier),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetStateIdentifier(
                    &self,
                    stateidentifier: &windows_core::HSTRING,
                ) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .SetStateIdentifier)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(stateidentifier),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetUserSettings(
                    &self,
                    settingscontent: &windows_core::HSTRING,
                ) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .SetUserSettings)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(settingscontent),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn CanUnloadPreference(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageManagerSettings2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CanUnloadPreference)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetCanUnloadPreference(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageManagerSettings2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetCanUnloadPreference)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn TerminationSignalMonitoring(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IPackageManagerSettings2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .TerminationSignalMonitoring)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetTerminationSignalMonitoring(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IPackageManagerSettings2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetTerminationSignalMonitoring)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for PackageManagerSettings {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageManagerSettings,
                >();
            }
            unsafe impl windows_core::Interface for PackageManagerSettings {
                type Vtable = <IPackageManagerSettings as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageManagerSettings as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageManagerSettings {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageManagerSettings";
            }
            unsafe impl Send for PackageManagerSettings {}
            unsafe impl Sync for PackageManagerSettings {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageMatchField(pub i32);
            impl PackageMatchField {
                pub const CatalogDefault: Self = Self(0);
                pub const Id: Self = Self(1);
                pub const Name: Self = Self(2);
                pub const Moniker: Self = Self(3);
                pub const Command: Self = Self(4);
                pub const Tag: Self = Self(5);
                pub const PackageFamilyName: Self = Self(6);
                pub const ProductCode: Self = Self(7);
            }
            impl windows_core::imp::TypeKind for PackageMatchField {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageMatchField {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageMatchField;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageMatchField",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageMatchFilter(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageMatchFilter, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageMatchFilter {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        PackageMatchFilter,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Option(&self) -> windows_core::Result<PackageFieldMatchOption> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Option)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetOption(
                    &self,
                    value: PackageFieldMatchOption,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetOption)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn Field(&self) -> windows_core::Result<PackageMatchField> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Field)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetField(
                    &self,
                    value: PackageMatchField,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetField)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Value)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetValue(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetValue)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for PackageMatchFilter {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageMatchFilter,
                >();
            }
            unsafe impl windows_core::Interface for PackageMatchFilter {
                type Vtable = <IPackageMatchFilter as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageMatchFilter as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageMatchFilter {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageMatchFilter";
            }
            unsafe impl Send for PackageMatchFilter {}
            unsafe impl Sync for PackageMatchFilter {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageRepairMode(pub i32);
            impl PackageRepairMode {
                pub const Default: Self = Self(0);
                pub const Silent: Self = Self(1);
                pub const Interactive: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageRepairMode {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageRepairMode {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageRepairMode;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageRepairMode",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageRepairProgressState(pub i32);
            impl PackageRepairProgressState {
                pub const Queued: Self = Self(0);
                pub const Repairing: Self = Self(1);
                pub const PostRepair: Self = Self(2);
                pub const Finished: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for PackageRepairProgressState {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageRepairProgressState {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageRepairProgressState;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageRepairProgressState",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageRepairScope(pub i32);
            impl PackageRepairScope {
                pub const Any: Self = Self(0);
                pub const User: Self = Self(1);
                pub const System: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageRepairScope {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageRepairScope {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageRepairScope;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageRepairScope",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageUninstallMode(pub i32);
            impl PackageUninstallMode {
                pub const Default: Self = Self(0);
                pub const Silent: Self = Self(1);
                pub const Interactive: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageUninstallMode {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageUninstallMode {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageUninstallMode;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageUninstallMode",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageUninstallProgressState(pub i32);
            impl PackageUninstallProgressState {
                pub const Queued: Self = Self(0);
                pub const Uninstalling: Self = Self(1);
                pub const PostUninstall: Self = Self(2);
                pub const Finished: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for PackageUninstallProgressState {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageUninstallProgressState {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageUninstallProgressState;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageUninstallProgressState",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageUninstallScope(pub i32);
            impl PackageUninstallScope {
                pub const Any: Self = Self(0);
                pub const User: Self = Self(1);
                pub const System: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for PackageUninstallScope {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageUninstallScope {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageUninstallScope;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageUninstallScope",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageVersionId(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageVersionId, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageVersionId {
                pub fn PackageCatalogId(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageCatalogId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Version)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Channel(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Channel)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageVersionId {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageVersionId,
                >();
            }
            unsafe impl windows_core::Interface for PackageVersionId {
                type Vtable = <IPackageVersionId as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageVersionId as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageVersionId {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageVersionId";
            }
            unsafe impl Send for PackageVersionId {}
            unsafe impl Sync for PackageVersionId {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PackageVersionInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PackageVersionInfo, windows_core::IUnknown, windows_core::IInspectable
            );
            impl PackageVersionInfo {
                pub fn GetMetadata(
                    &self,
                    metadatafield: PackageVersionMetadataField,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .GetMetadata)(
                                windows_core::Interface::as_raw(self),
                                metadatafield,
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Id)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn DisplayName(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .DisplayName)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Version)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Channel(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Channel)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn PackageFamilyNames(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<windows_core::HSTRING>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageFamilyNames)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ProductCodes(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IVectorView<windows_core::HSTRING>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ProductCodes)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn PackageCatalog(&self) -> windows_core::Result<PackageCatalog> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageCatalog)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CompareToVersion(
                    &self,
                    versionstring: &windows_core::HSTRING,
                ) -> windows_core::Result<CompareResult> {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CompareToVersion)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(versionstring),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn HasApplicableInstaller<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<bool>
                where
                    P0: windows_core::Param<InstallOptions>,
                {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .HasApplicableInstaller)(
                                windows_core::Interface::as_raw(this),
                                options.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Publisher(&self) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo3,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Publisher)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn GetCatalogPackageMetadata(
                    &self,
                ) -> windows_core::Result<CatalogPackageMetadata> {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo4,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetCatalogPackageMetadata)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetCatalogPackageMetadata2(
                    &self,
                    preferredlocale: &windows_core::HSTRING,
                ) -> windows_core::Result<CatalogPackageMetadata> {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo4,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetCatalogPackageMetadata2)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(preferredlocale),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetApplicableInstaller<P0>(
                    &self,
                    options: P0,
                ) -> windows_core::Result<PackageInstallerInfo>
                where
                    P0: windows_core::Param<InstallOptions>,
                {
                    let this = &windows_core::Interface::cast::<
                        IPackageVersionInfo4,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .GetApplicableInstaller)(
                                windows_core::Interface::as_raw(this),
                                options.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for PackageVersionInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IPackageVersionInfo,
                >();
            }
            unsafe impl windows_core::Interface for PackageVersionInfo {
                type Vtable = <IPackageVersionInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPackageVersionInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PackageVersionInfo {
                const NAME: &'static str = "Microsoft.Management.Deployment.PackageVersionInfo";
            }
            unsafe impl Send for PackageVersionInfo {}
            unsafe impl Sync for PackageVersionInfo {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PackageVersionMetadataField(pub i32);
            impl PackageVersionMetadataField {
                pub const InstallerType: Self = Self(0);
                pub const InstalledScope: Self = Self(1);
                pub const InstalledLocation: Self = Self(2);
                pub const StandardUninstallCommand: Self = Self(3);
                pub const SilentUninstallCommand: Self = Self(4);
                pub const PublisherDisplayName: Self = Self(5);
            }
            impl windows_core::imp::TypeKind for PackageVersionMetadataField {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PackageVersionMetadataField {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PackageVersionMetadataField;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PackageVersionMetadataField",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct PredefinedPackageCatalog(pub i32);
            impl PredefinedPackageCatalog {
                pub const OpenWindowsCatalog: Self = Self(0);
                pub const MicrosoftStore: Self = Self(1);
                pub const DesktopFrameworks: Self = Self(2);
                pub const OpenWindowsCatalogFont: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for PredefinedPackageCatalog {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for PredefinedPackageCatalog {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.PredefinedPackageCatalog;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.PredefinedPackageCatalog",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RefreshPackageCatalogResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                RefreshPackageCatalogResult, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl RefreshPackageCatalogResult {
                pub fn Status(
                    &self,
                ) -> windows_core::Result<RefreshPackageCatalogStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for RefreshPackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IRefreshPackageCatalogResult,
                >();
            }
            unsafe impl windows_core::Interface for RefreshPackageCatalogResult {
                type Vtable = <IRefreshPackageCatalogResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRefreshPackageCatalogResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RefreshPackageCatalogResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.RefreshPackageCatalogResult";
            }
            unsafe impl Send for RefreshPackageCatalogResult {}
            unsafe impl Sync for RefreshPackageCatalogResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct RefreshPackageCatalogStatus(pub i32);
            impl RefreshPackageCatalogStatus {
                pub const Ok: Self = Self(0);
                pub const GroupPolicyError: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
            }
            impl windows_core::imp::TypeKind for RefreshPackageCatalogStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for RefreshPackageCatalogStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.RefreshPackageCatalogStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.RefreshPackageCatalogStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RemovePackageCatalogOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                RemovePackageCatalogOptions, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl RemovePackageCatalogOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        RemovePackageCatalogOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Name)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetName(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetName)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn PreserveData(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PreserveData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPreserveData(&self, value: bool) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPreserveData)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for RemovePackageCatalogOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IRemovePackageCatalogOptions,
                >();
            }
            unsafe impl windows_core::Interface for RemovePackageCatalogOptions {
                type Vtable = <IRemovePackageCatalogOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRemovePackageCatalogOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RemovePackageCatalogOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.RemovePackageCatalogOptions";
            }
            unsafe impl Send for RemovePackageCatalogOptions {}
            unsafe impl Sync for RemovePackageCatalogOptions {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RemovePackageCatalogResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                RemovePackageCatalogResult, windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl RemovePackageCatalogResult {
                pub fn Status(
                    &self,
                ) -> windows_core::Result<RemovePackageCatalogStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for RemovePackageCatalogResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IRemovePackageCatalogResult,
                >();
            }
            unsafe impl windows_core::Interface for RemovePackageCatalogResult {
                type Vtable = <IRemovePackageCatalogResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRemovePackageCatalogResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RemovePackageCatalogResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.RemovePackageCatalogResult";
            }
            unsafe impl Send for RemovePackageCatalogResult {}
            unsafe impl Sync for RemovePackageCatalogResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct RemovePackageCatalogStatus(pub i32);
            impl RemovePackageCatalogStatus {
                pub const Ok: Self = Self(0);
                pub const GroupPolicyError: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const AccessDenied: Self = Self(4);
                pub const InvalidOptions: Self = Self(5);
            }
            impl windows_core::imp::TypeKind for RemovePackageCatalogStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for RemovePackageCatalogStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.RemovePackageCatalogStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.RemovePackageCatalogStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RepairOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                RepairOptions, windows_core::IUnknown, windows_core::IInspectable
            );
            impl RepairOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        RepairOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn PackageVersionId(
                    &self,
                ) -> windows_core::Result<PackageVersionId> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetPackageVersionId<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<PackageVersionId>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn PackageRepairScope(
                    &self,
                ) -> windows_core::Result<PackageRepairScope> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageRepairScope)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageRepairScope(
                    &self,
                    value: PackageRepairScope,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageRepairScope)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn PackageRepairMode(
                    &self,
                ) -> windows_core::Result<PackageRepairMode> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageRepairMode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageRepairMode(
                    &self,
                    value: PackageRepairMode,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageRepairMode)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AcceptPackageAgreements(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AcceptPackageAgreements)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAcceptPackageAgreements(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAcceptPackageAgreements)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetCorrelationData(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCorrelationData)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn AllowHashMismatch(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .AllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetAllowHashMismatch(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetAllowHashMismatch)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn LogOutputPath(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .LogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetLogOutputPath(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetLogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn Force(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Force)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetForce(&self, value: bool) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetForce)(windows_core::Interface::as_raw(self), value)
                            .ok()
                    }
                }
                pub fn BypassIsStoreClientBlockedPolicyCheck(
                    &self,
                ) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .BypassIsStoreClientBlockedPolicyCheck)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetBypassIsStoreClientBlockedPolicyCheck(
                    &self,
                    value: bool,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetBypassIsStoreClientBlockedPolicyCheck)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn AuthenticationArguments(
                    &self,
                ) -> windows_core::Result<AuthenticationArguments> {
                    let this = &windows_core::Interface::cast::<IRepairOptions2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .AuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetAuthenticationArguments<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<AuthenticationArguments>,
                {
                    let this = &windows_core::Interface::cast::<IRepairOptions2>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetAuthenticationArguments)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for RepairOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IRepairOptions,
                >();
            }
            unsafe impl windows_core::Interface for RepairOptions {
                type Vtable = <IRepairOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRepairOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RepairOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.RepairOptions";
            }
            unsafe impl Send for RepairOptions {}
            unsafe impl Sync for RepairOptions {}
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct RepairProgress {
                pub State: PackageRepairProgressState,
                pub RepairCompletionProgress: f64,
            }
            impl windows_core::imp::TypeKind for RepairProgress {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for RepairProgress {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Microsoft.Management.Deployment.RepairProgress;enum(Microsoft.Management.Deployment.PackageRepairProgressState;i4);f8)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.RepairProgress",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RepairResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                RepairResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl RepairResult {
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn RebootRequired(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .RebootRequired)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Status(&self) -> windows_core::Result<RepairResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn RepairerErrorCode(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .RepairerErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for RepairResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IRepairResult,
                >();
            }
            unsafe impl windows_core::Interface for RepairResult {
                type Vtable = <IRepairResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRepairResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RepairResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.RepairResult";
            }
            unsafe impl Send for RepairResult {}
            unsafe impl Sync for RepairResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct RepairResultStatus(pub i32);
            impl RepairResultStatus {
                pub const Ok: Self = Self(0);
                pub const BlockedByPolicy: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const DownloadError: Self = Self(3);
                pub const InternalError: Self = Self(4);
                pub const InvalidOptions: Self = Self(5);
                pub const RepairError: Self = Self(6);
                pub const ManifestError: Self = Self(7);
                pub const NoApplicableRepairer: Self = Self(8);
                pub const PackageAgreementsNotAccepted: Self = Self(9);
            }
            impl windows_core::imp::TypeKind for RepairResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for RepairResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.RepairResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.RepairResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct SourceAgreement(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                SourceAgreement, windows_core::IUnknown, windows_core::IInspectable
            );
            impl SourceAgreement {
                pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Label)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Text)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Url(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Url)(windows_core::Interface::as_raw(self), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for SourceAgreement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    ISourceAgreement,
                >();
            }
            unsafe impl windows_core::Interface for SourceAgreement {
                type Vtable = <ISourceAgreement as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ISourceAgreement as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for SourceAgreement {
                const NAME: &'static str = "Microsoft.Management.Deployment.SourceAgreement";
            }
            unsafe impl Send for SourceAgreement {}
            unsafe impl Sync for SourceAgreement {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UninstallOptions(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UninstallOptions, windows_core::IUnknown, windows_core::IInspectable
            );
            impl UninstallOptions {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(
                            &windows_core::imp::IGenericFactory,
                        ) -> windows_core::Result<R>,
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        UninstallOptions,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn PackageVersionId(
                    &self,
                ) -> windows_core::Result<PackageVersionId> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetPackageVersionId<P0>(
                    &self,
                    value: P0,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<PackageVersionId>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageVersionId)(
                                windows_core::Interface::as_raw(self),
                                value.param().abi(),
                            )
                            .ok()
                    }
                }
                pub fn PackageUninstallMode(
                    &self,
                ) -> windows_core::Result<PackageUninstallMode> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .PackageUninstallMode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageUninstallMode(
                    &self,
                    value: PackageUninstallMode,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetPackageUninstallMode)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                    }
                }
                pub fn LogOutputPath(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .LogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetLogOutputPath(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetLogOutputPath)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetCorrelationData(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self)
                            .SetCorrelationData)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                    }
                }
                pub fn Force(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        IUninstallOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .Force)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetForce(&self, value: bool) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IUninstallOptions2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetForce)(windows_core::Interface::as_raw(this), value)
                            .ok()
                    }
                }
                pub fn PackageUninstallScope(
                    &self,
                ) -> windows_core::Result<PackageUninstallScope> {
                    let this = &windows_core::Interface::cast::<
                        IUninstallOptions2,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .PackageUninstallScope)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn SetPackageUninstallScope(
                    &self,
                    value: PackageUninstallScope,
                ) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        IUninstallOptions2,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this)
                            .SetPackageUninstallScope)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                    }
                }
            }
            impl windows_core::RuntimeType for UninstallOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IUninstallOptions,
                >();
            }
            unsafe impl windows_core::Interface for UninstallOptions {
                type Vtable = <IUninstallOptions as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IUninstallOptions as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UninstallOptions {
                const NAME: &'static str = "Microsoft.Management.Deployment.UninstallOptions";
            }
            unsafe impl Send for UninstallOptions {}
            unsafe impl Sync for UninstallOptions {}
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct UninstallProgress {
                pub State: PackageUninstallProgressState,
                pub UninstallationProgress: f64,
            }
            impl windows_core::imp::TypeKind for UninstallProgress {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for UninstallProgress {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Microsoft.Management.Deployment.UninstallProgress;enum(Microsoft.Management.Deployment.PackageUninstallProgressState;i4);f8)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.UninstallProgress",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UninstallResult(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UninstallResult, windows_core::IUnknown, windows_core::IInspectable
            );
            impl UninstallResult {
                pub fn CorrelationData(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .CorrelationData)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn RebootRequired(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .RebootRequired)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn Status(&self) -> windows_core::Result<UninstallResultStatus> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .Status)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn ExtendedErrorCode(
                    &self,
                ) -> windows_core::Result<windows_core::HRESULT> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .ExtendedErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
                pub fn UninstallerErrorCode(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self)
                            .UninstallerErrorCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for UninstallResult {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                    Self,
                    IUninstallResult,
                >();
            }
            unsafe impl windows_core::Interface for UninstallResult {
                type Vtable = <IUninstallResult as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IUninstallResult as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UninstallResult {
                const NAME: &'static str = "Microsoft.Management.Deployment.UninstallResult";
            }
            unsafe impl Send for UninstallResult {}
            unsafe impl Sync for UninstallResult {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct UninstallResultStatus(pub i32);
            impl UninstallResultStatus {
                pub const Ok: Self = Self(0);
                pub const BlockedByPolicy: Self = Self(1);
                pub const CatalogError: Self = Self(2);
                pub const InternalError: Self = Self(3);
                pub const InvalidOptions: Self = Self(4);
                pub const UninstallError: Self = Self(5);
                pub const ManifestError: Self = Self(6);
            }
            impl windows_core::imp::TypeKind for UninstallResultStatus {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for UninstallResultStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.UninstallResultStatus;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.UninstallResultStatus",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct WindowsPlatform(pub i32);
            impl WindowsPlatform {
                pub const Unknown: Self = Self(0);
                pub const Universal: Self = Self(1);
                pub const Desktop: Self = Self(2);
                pub const IoT: Self = Self(3);
                pub const Team: Self = Self(4);
                pub const Holographic: Self = Self(5);
            }
            impl windows_core::imp::TypeKind for WindowsPlatform {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for WindowsPlatform {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Microsoft.Management.Deployment.WindowsPlatform;i4)",
                );
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"Microsoft.Management.Deployment.WindowsPlatform",
                );
            }
        }
    }
}
pub mod Windows {
    pub mod System {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct ProcessorArchitecture(pub i32);
        impl ProcessorArchitecture {
            pub const X86: Self = Self(0);
            pub const Arm: Self = Self(5);
            pub const X64: Self = Self(9);
            pub const Neutral: Self = Self(11);
            pub const Arm64: Self = Self(12);
            pub const X86OnArm64: Self = Self(14);
            pub const Unknown: Self = Self(65535);
        }
        impl windows_core::imp::TypeKind for ProcessorArchitecture {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for ProcessorArchitecture {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"enum(Windows.System.ProcessorArchitecture;i4)",
            );
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.System.ProcessorArchitecture",
            );
        }
    }
}
